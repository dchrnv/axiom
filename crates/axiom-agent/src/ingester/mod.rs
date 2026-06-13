// ingester/mod.rs — FileIngester: загрузка файлов и конвертация в UCL команды.
//
// Архитектура (per Opus, 2026-06-13):
//   FileIngester = «рот» (перцепция, axiom-agent)
//   Seed Compiler = «желудок» (axiom-seed, будущее)
//   Сейчас: FileIngester → TextPerceptor → UCL команды
//
// Инвариант: нет второго пути в SUTRA. Все чанки идут через TextPerceptor.

pub mod dataset;
pub mod markdown;

use std::path::Path;
use std::sync::Arc;

use axiom_config::AnchorSet;
use axiom_ucl::UclCommand;

pub use dataset::{AxiomDataset, Chunk, ChunkKind, InjectMode};
pub use markdown::parse_markdown;

use crate::perceptors::text::TextPerceptor;

/// Результат инжестирования одного файла/датасета.
#[derive(Debug, Default)]
pub struct IngestResult {
    /// Количество чанков обработано
    pub chunks_total: usize,
    /// Количество UCL команд сгенерировано
    pub commands_total: usize,
    /// Предупреждения (расхождения subsystem_hint vs детекция)
    pub hints_mismatch: Vec<String>,
}

/// Загрузчик файлов в AXIOM.
///
/// Читает .md или .axiom.yaml → чанки → UCL команды через TextPerceptor.
pub struct FileIngester {
    perceptor: TextPerceptor,
}

impl FileIngester {
    pub fn new() -> Self {
        Self { perceptor: TextPerceptor::new() }
    }

    pub fn with_anchors(anchors: Arc<AnchorSet>) -> Self {
        Self { perceptor: TextPerceptor::with_anchors(anchors) }
    }

    /// Загрузить .md файл и конвертировать в UCL команды.
    pub fn load_md(
        &self,
        path: &Path,
    ) -> Result<(Vec<UclCommand>, IngestResult), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let chunks = parse_markdown(&content, InjectMode::Grow);
        Ok(self.chunks_to_commands(chunks))
    }

    /// Загрузить .axiom.yaml датасет и конвертировать в UCL команды.
    pub fn load_dataset(
        &self,
        path: &Path,
    ) -> Result<(Vec<UclCommand>, IngestResult), String> {
        let ds = AxiomDataset::from_yaml(path)?;
        let chunks = ds.to_chunks();
        Ok(self.chunks_to_commands(chunks))
    }

    /// Preview .md файла без инъекции.
    pub fn dry_run_md(&self, path: &Path) -> Result<DryRunReport, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let chunks = parse_markdown(&content, InjectMode::Grow);
        Ok(self.build_dry_report(&chunks))
    }

    /// Конвертировать чанки в UCL команды.
    ///
    /// Для каждого paragraph чанка с section_header: добавляем BondTokens
    /// (COMPOSITION) к заголовку секции. Это даёт FrameWeaver иерархию.
    fn chunks_to_commands(&self, chunks: Vec<Chunk>) -> (Vec<UclCommand>, IngestResult) {
        let mut commands = Vec::new();
        let mut result = IngestResult::default();

        for chunk in &chunks {
            if chunk.is_empty() {
                continue;
            }
            result.chunks_total += 1;

            // Верифицируем subsystem_hint
            if let Some(ref hint) = chunk.subsystem_hint {
                if let Some(detected) = self.perceptor.detect_subsystem(&chunk.content) {
                    if &detected != hint {
                        result.hints_mismatch.push(format!(
                            "hint={hint} detected={detected} content=\"{}\"",
                            &chunk.content[..chunk.content.len().min(40)]
                        ));
                    }
                }
            }

            // Основной токен
            let mut cmds = self.perceptor.perceive_and_bond(&chunk.content);

            // Composition bond: paragraph → section header
            if chunk.kind == ChunkKind::Paragraph {
                if let Some(ref header) = chunk.section_header {
                    // Добавляем bond к заголовку секции
                    let header_cmds = self.perceptor.perceive_and_bond(header);
                    // Bond: paragraph_token → header_token (composition)
                    // perceive_and_bond уже создаёт bonds к якорям;
                    // здесь дополнительно создаём section structure bond
                    // через stable_id paragraph ↔ stable_id header
                    if let (Some(para_cmd), Some(hdr_cmd)) =
                        (cmds.first(), header_cmds.first())
                    {
                        use axiom_ucl::{BondTokensPayload, OpCode};
                        use axiom_core::FLAG_ACTIVE;
                        use axiom_shell::link_types;

                        let para_id = text_stable_id(&chunk.content);
                        let hdr_id = text_stable_id(header);

                        if para_id != hdr_id {
                            let bond = BondTokensPayload {
                                source_id: para_id,
                                target_id: hdr_id,
                                domain_id: 110, // MAYA
                                link_type: link_types::COMPOSITION_BOND,
                                strength: 0.9,
                                conn_flags: FLAG_ACTIVE,
                                origin_domain: 100, // SUTRA
                                role_id: 0,
                                reserved: [0; 24],
                            };
                            let _ = (para_cmd, hdr_cmd); // suppress unused warning
                            cmds.push(
                                UclCommand::new(OpCode::BondTokens, 0, 10, 0)
                                    .with_payload(&bond),
                            );
                        }
                    }
                }
            }

            result.commands_total += cmds.len();
            commands.extend(cmds);
        }

        (commands, result)
    }

    fn build_dry_report(&self, chunks: &[Chunk]) -> DryRunReport {
        let mut report = DryRunReport::default();
        report.chunks_total = chunks.iter().filter(|c| !c.is_empty()).count();
        report.headers = chunks
            .iter()
            .filter(|c| c.kind == ChunkKind::Header)
            .map(|c| c.content.clone())
            .collect();
        for chunk in chunks {
            if chunk.is_empty() { continue; }
            if let Some(sub) = self.perceptor.detect_subsystem(&chunk.content) {
                *report.subsystems.entry(sub).or_insert(0) += 1;
            }
        }
        report
    }
}

impl Default for FileIngester {
    fn default() -> Self {
        Self::new()
    }
}

/// FNV-1a для stable_id (зеркало text.rs).
fn text_stable_id(text: &str) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in text.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let id = (h & 0x3FFF_FFFF) as u32;
    (id | 0x4000_0000).max(0x4000_0001)
}

/// Отчёт dry-run для :load-dry команды.
#[derive(Debug, Default)]
pub struct DryRunReport {
    pub chunks_total: usize,
    pub headers: Vec<String>,
    pub subsystems: std::collections::HashMap<String, usize>,
}

impl std::fmt::Display for DryRunReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "chunks: {}", self.chunks_total)?;
        writeln!(f, "sections: {}", self.headers.len())?;
        for h in &self.headers {
            writeln!(f, "  § {h}")?;
        }
        if !self.subsystems.is_empty() {
            writeln!(f, "detected subsystems:")?;
            let mut subs: Vec<_> = self.subsystems.iter().collect();
            subs.sort_by(|a, b| b.1.cmp(a.1));
            for (sub, count) in subs {
                writeln!(f, "  {sub}: {count}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_md_produces_commands() {
        let ingester = FileIngester::new();
        let md = "## Математика\n\nПростые числа бесконечны по количеству.";
        let chunks = parse_markdown(md, InjectMode::Grow);
        let (cmds, result) = ingester.chunks_to_commands(chunks);
        assert!(!cmds.is_empty(), "должны быть UCL команды");
        assert_eq!(result.chunks_total, 2); // заголовок + абзац
    }

    #[test]
    fn test_duplicate_content_same_commands() {
        let ingester = FileIngester::new();
        let md = "Тестовый текст.";
        let chunks1 = parse_markdown(md, InjectMode::Grow);
        let chunks2 = parse_markdown(md, InjectMode::Grow);
        let (cmds1, _) = ingester.chunks_to_commands(chunks1);
        let (cmds2, _) = ingester.chunks_to_commands(chunks2);
        // stable_id детерминирован → те же команды (дубликат = подкрепление)
        assert_eq!(cmds1.len(), cmds2.len());
        assert_eq!(cmds1[0].payload, cmds2[0].payload);
    }

    #[test]
    fn test_dry_run_no_commands() {
        let ingester = FileIngester::new();
        let chunks = parse_markdown("## Раздел\n\nТекст абзаца.", InjectMode::Grow);
        let report = ingester.build_dry_report(&chunks);
        assert_eq!(report.chunks_total, 2);
        assert_eq!(report.headers, vec!["Раздел"]);
    }
}
