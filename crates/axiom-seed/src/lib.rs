// axiom-seed — Crystal Layout бэкенд Seed Compiler.
//
// Стартовая библиотека якорей: charset + region → позиции кристалла.
// Детерминизм: charset + region → одинаковый кристалл всегда.
#![deny(unsafe_code)]

pub mod charset;
pub mod compiler;
pub mod layout;
