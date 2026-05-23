/// TextPerceptor — преобразует строку UTF-8 в UclCommand(InjectToken)
pub mod text;
/// Символьный/словарный матчинг текста к якорным примитивам (E1 путь А)
pub mod anchor_match;
/// Таблица декомпозиции: char/word → якорный ID
pub mod decomposition_table;
