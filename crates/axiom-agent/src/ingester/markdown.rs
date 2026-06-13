// markdown.rs — .md → Vec<Chunk> (чанкинг по секциям, Вариант Б).
//
// Структура:
//   # / ## / ### строки → Header чанки (начинают новую секцию)
//   Непустые абзацы → Paragraph чанки с section_header = текущий заголовок
//   Пустые строки — разделители (не чанки)
//
// Связи (задаёт ingester/mod.rs через perceive_and_bond):
//   COMPOSITION bond: заголовок секции ↔ каждый её абзац

use super::dataset::{Chunk, ChunkKind, InjectMode};

/// Парсить Markdown текст в список чанков.
///
/// Возвращает чанки в порядке документа. Header чанки предшествуют своим абзацам.
pub fn parse_markdown(text: &str, default_mode: InjectMode) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mut current_section: Option<String> = None;
    let mut paragraph_buf = String::new();

    let flush_paragraph = |buf: &mut String, section: &Option<String>, chunks: &mut Vec<Chunk>, mode: InjectMode| {
        let trimmed = buf.trim().to_string();
        if !trimmed.is_empty() {
            chunks.push(Chunk {
                content: trimmed,
                kind: ChunkKind::Paragraph,
                section_header: section.clone(),
                tags: Vec::new(),
                subsystem_hint: None,
                inject_mode: mode,
            });
        }
        buf.clear();
    };

    for line in text.lines() {
        if let Some(header_text) = extract_header(line) {
            // Сбросить накопленный абзац
            flush_paragraph(&mut paragraph_buf, &current_section, &mut chunks, default_mode);

            // Добавить заголовок секции
            chunks.push(Chunk {
                content: header_text.clone(),
                kind: ChunkKind::Header,
                section_header: None,
                tags: Vec::new(),
                subsystem_hint: None,
                inject_mode: default_mode,
            });
            current_section = Some(header_text);
        } else if line.trim().is_empty() {
            // Пустая строка — flush абзаца
            flush_paragraph(&mut paragraph_buf, &current_section, &mut chunks, default_mode);
        } else {
            // Продолжение абзаца
            let stripped = strip_markdown_inline(line);
            if !paragraph_buf.is_empty() {
                paragraph_buf.push(' ');
            }
            paragraph_buf.push_str(&stripped);
        }
    }

    // Flush последнего абзаца
    flush_paragraph(&mut paragraph_buf, &current_section, &mut chunks, default_mode);

    chunks
}

/// Извлечь текст заголовка из строки Markdown (#, ##, ###, ####).
/// Возвращает None если строка не является заголовком.
fn extract_header(line: &str) -> Option<String> {
    let stripped = line.trim_start_matches('#');
    let hashes = line.len() - stripped.len();
    if hashes >= 1 && hashes <= 6 && stripped.starts_with(' ') {
        Some(stripped.trim().to_string())
    } else {
        None
    }
}

/// Убрать простую Markdown-разметку из строки (bold, italic, code, links).
///
/// Обрабатывает: **bold**, *italic*, `code`, [text](url).
/// Не парсит вложенную разметку — достаточно для текстовой инъекции.
fn strip_markdown_inline(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // **bold** или __bold__
        if (bytes[i] == b'*' || bytes[i] == b'_')
            && i + 1 < bytes.len()
            && bytes[i + 1] == bytes[i]
        {
            let marker = bytes[i];
            i += 2;
            let start = i;
            while i + 1 < bytes.len() && !(bytes[i] == marker && bytes[i + 1] == marker) {
                i += 1;
            }
            out.push_str(std::str::from_utf8(&bytes[start..i]).unwrap_or(""));
            if i + 1 < bytes.len() { i += 2; } // skip closing **
        }
        // *italic* или _italic_
        else if bytes[i] == b'*' || bytes[i] == b'_' {
            let marker = bytes[i];
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != marker {
                i += 1;
            }
            out.push_str(std::str::from_utf8(&bytes[start..i]).unwrap_or(""));
            if i < bytes.len() { i += 1; }
        }
        // `code`
        else if bytes[i] == b'`' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b'`' {
                i += 1;
            }
            out.push_str(std::str::from_utf8(&bytes[start..i]).unwrap_or(""));
            if i < bytes.len() { i += 1; }
        }
        // [text](url) → text
        else if bytes[i] == b'[' {
            i += 1;
            let start = i;
            while i < bytes.len() && bytes[i] != b']' {
                i += 1;
            }
            out.push_str(std::str::from_utf8(&bytes[start..i]).unwrap_or(""));
            if i < bytes.len() { i += 1; } // skip ]
            if i < bytes.len() && bytes[i] == b'(' {
                i += 1;
                while i < bytes.len() && bytes[i] != b')' { i += 1; }
                if i < bytes.len() { i += 1; }
            }
        }
        else {
            // Безопасно копируем UTF-8 символ
            let ch_len = utf8_char_len(bytes[i]);
            out.push_str(std::str::from_utf8(&bytes[i..i + ch_len.min(bytes.len() - i)]).unwrap_or(""));
            i += ch_len;
        }
    }

    out
}

fn utf8_char_len(b: u8) -> usize {
    if b < 0x80 { 1 }
    else if b < 0xE0 { 2 }
    else if b < 0xF0 { 3 }
    else { 4 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sections() {
        let md = "## Математика\n\nПростые числа бесконечны.\n\nЭто доказал Евклид.\n\n## Логика\n\nЕсли A то B.";
        let chunks = parse_markdown(md, InjectMode::Grow);

        let headers: Vec<_> = chunks.iter().filter(|c| c.kind == ChunkKind::Header).collect();
        let paragraphs: Vec<_> = chunks.iter().filter(|c| c.kind == ChunkKind::Paragraph).collect();

        assert_eq!(headers.len(), 2);
        assert_eq!(paragraphs.len(), 3);
        assert_eq!(headers[0].content, "Математика");
        assert_eq!(headers[1].content, "Логика");
    }

    #[test]
    fn test_section_header_attached_to_paragraphs() {
        let md = "## Раздел\n\nАбзац первый.\n\nАбзац второй.";
        let chunks = parse_markdown(md, InjectMode::Grow);

        let paragraphs: Vec<_> = chunks.iter().filter(|c| c.kind == ChunkKind::Paragraph).collect();
        assert_eq!(paragraphs.len(), 2);
        assert_eq!(paragraphs[0].section_header.as_deref(), Some("Раздел"));
        assert_eq!(paragraphs[1].section_header.as_deref(), Some("Раздел"));
    }

    #[test]
    fn test_no_headers_all_paragraphs() {
        let md = "Первый абзац.\n\nВторой абзац.";
        let chunks = parse_markdown(md, InjectMode::Grow);
        assert_eq!(chunks.len(), 2);
        assert!(chunks.iter().all(|c| c.kind == ChunkKind::Paragraph));
        assert!(chunks[0].section_header.is_none());
    }

    #[test]
    fn test_strip_bold_italic() {
        let md = "**жирный** и *курсив* и `код`";
        let chunks = parse_markdown(md, InjectMode::Grow);
        assert_eq!(chunks.len(), 1);
        assert!(chunks[0].content.contains("жирный"));
        assert!(chunks[0].content.contains("курсив"));
        assert!(chunks[0].content.contains("код"));
        assert!(!chunks[0].content.contains('*'));
        assert!(!chunks[0].content.contains('`'));
    }

    #[test]
    fn test_empty_document() {
        let chunks = parse_markdown("", InjectMode::Grow);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_header_levels_all_detected() {
        let md = "# H1\n## H2\n### H3\n#### H4";
        let chunks = parse_markdown(md, InjectMode::Grow);
        assert_eq!(chunks.iter().filter(|c| c.kind == ChunkKind::Header).count(), 4);
    }

    #[test]
    fn test_multiline_paragraph_joined() {
        let md = "## Sec\n\nСтрока первая\nСтрока вторая";
        let chunks = parse_markdown(md, InjectMode::Grow);
        let para = chunks.iter().find(|c| c.kind == ChunkKind::Paragraph).unwrap();
        assert!(para.content.contains("первая"));
        assert!(para.content.contains("вторая"));
    }
}
