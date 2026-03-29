/// CLI-канал (stdin/stdout)
pub mod cli;
/// Telegram-канал (Bot API)
pub mod telegram;
/// Shell Effector (whitelist-защищённое выполнение команд)
pub mod shell;
/// Vision Perceptor (изображение → токены через ML)
pub mod vision;
/// Audio Perceptor (речь → токены через VAD)
pub mod audio;
