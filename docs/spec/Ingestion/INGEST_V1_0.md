# AXIOM — File Ingestion V1.0

**Статус:** Спецификация  
**Дата:** 2026-06-13  
**Crate:** `axiom-agent` (ingester/) + `axiom-agent` (dataset.rs)  
**Опирается на:** INGESTION_CONCEPT.md (решения Opus), TextPerceptor, axiom-seed (будущий Seed Compiler)

---

## 0. Назначение

FileIngester — новый «рот» системы. Читает .md файлы и AxiomDataset (.axiom.yaml),
превращает в поток UCL команд через TextPerceptor. Stomach (Seed Injection) — позже.

```
.md / .axiom.yaml
      ↓
  FileIngester       ← axiom-agent/src/ingester/
      ↓
  chunks: Vec<Chunk>
      ↓
  TextPerceptor::perceive_and_bond
      ↓
  Vec<UclCommand>    → engine (SUTRA → MAYA → Experience)
```

---

## 1. AxiomDataset формат (.axiom.yaml)

```yaml
metadata:
  name: "dataset_name"
  version: "1.0"
  inject_mode: "grow"       # grow (дефолт) | anchor
  ticks_between: 50         # пауза между чанками (тиков)
  subsystem_hint: "logic"   # опц. — только для верификации, не директива

entries:
  - id: "entry_id"
    content: "Текст чанка..."
    tags: ["tag1", "tag2"]
    subsystem: "abstractions"   # верификационный, не директивный в grow-режиме
```

**Правила:**
- `inject_mode: grow` — дефолт. Токены через MAYA, кристаллизуются, могут забыться.
- `inject_mode: anchor` — только осознанно. STATE_LOCKED, boot/DREAMING, инвариант 11.
- `subsystem` в grow — проверочный: система сама определяет, hint пишем в лог при расхождении.
- Дубликаты (same stable_id) = подкрепление, не ошибка.

---

## 2. .md парсер — чанкинг по секциям (Вариант Б)

```
# Заголовок документа   → HeaderChunk (mass+++)
## Раздел 1             → SectionChunk (заголовок раздела)
Абзац 1...              → ParagraphChunk
Абзац 2...              → ParagraphChunk
                        → COMPOSITION bonds: SectionChunk ↔ каждый ParagraphChunk
## Раздел 2             → новая секция, свои bonds
```

**Алгоритм:**
1. Разбиваем по строкам.
2. Строки `# ` / `## ` / `### ` → новая секция, строка становится header chunk.
3. Непустые абзацы → paragraph chunks внутри текущей секции.
4. Для каждой секции: BondTokens header ↔ paragraph (link_type COMPOSITION, strength 0.9).
5. Пустые строки — разделители, не чанки.

**Chunk структура:**
```rust
pub struct Chunk {
    pub content: String,
    pub kind: ChunkKind,        // Header | Paragraph
    pub section_id: Option<u32>, // stable_id заголовка секции
    pub tags: Vec<String>,
}
```

---

## 3. Структура кода

```
crates/axiom-agent/src/
  ingester/
    mod.rs          — FileIngester: load_md(), load_dataset(), inject_chunks()
    markdown.rs     — parse_markdown() → Vec<Chunk>
    dataset.rs      — AxiomDataset: serde_yaml десериализация
  channels/
    cli.rs          — +:load команда (уже существует, добавить ветку)
```

---

## 4. CLI команда

```
:load path/to/file.md              → загрузить .md, инжектировать
:load path/to/dataset.axiom.yaml   → загрузить датасет
:load-dry path/to/file.md          → preview (N чанков, подсистемы, без инъекции)
```

---

## 5. Инвариантые ограничения

- `anchor` режим — только при явном `inject_mode: anchor` в датасете. Не по умолчанию.
- SubsystemGravity не различает источник — нет специальных правил для файловых токенов.
- stable_id детерминирован от content → повтор = подкрепление.
- Нет стены в SUTRA: FileIngester не создаёт второго пути, только поставляет чанки в TextPerceptor.

---

## 6. Тесты

```
test_markdown_parse_sections       — секции разбиваются правильно
test_markdown_bonds_created        — header ↔ paragraphs bonds в output
test_dataset_load_grow_default     — inject_mode=grow если не задан
test_dataset_subsystem_hint_log    — расхождение hint vs детекция → лог (не ошибка)
test_chunk_duplicate_no_error      — same content дважды → stable_id тот же, нет паники
test_load_dry_no_engine_change     — :load-dry не меняет engine state
```

---

## 7. Что НЕ делать в V1

- Не парсить .pdf / .docx (INGEST-TD-02, optional features).
- Не реализовывать anchor-инъекцию файлов (только grow в V1, anchor в V1.1).
- Не добавлять специальную гравитацию для файловых токенов.
- Не делать streaming в training_data.jsonl (V1.1, после нейронного обучения).
