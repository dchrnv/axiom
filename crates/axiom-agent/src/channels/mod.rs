/// Audio Perceptor (речь → токены через VAD)
pub mod audio;
/// CLI-канал (stdin/stdout)
pub mod cli;
/// Shell Effector (whitelist-защищённое выполнение команд)
pub mod shell;
/// Telegram-канал (Bot API)
pub mod telegram;
/// Vision Perceptor (изображение → токены через ML)
pub mod vision;
