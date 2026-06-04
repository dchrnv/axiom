/// TextPerceptor — преобразует строку UTF-8 в UclCommand(InjectToken)
pub mod text;
/// Символьный/словарный матчинг текста к якорным примитивам (E1 путь А)
pub mod anchor_match;
/// Таблица декомпозиции: char/word → якорный ID
pub mod decomposition_table;
/// L0VisionPerceptor — edge detection → L0 visual primitives → SUTRA (V7-E2)
pub mod vision_l0;
/// TemporalPerceptor — temporal markers in text → time_*-anchors → SUTRA (PRIM-TD-04)
pub mod temporal;
