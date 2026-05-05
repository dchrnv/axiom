use crate::protocol::{DomainSummary, TokenSnapshot};

/// Состояние дашборда — обновляется WS-потоком, читается GUI.
#[derive(Default)]
pub struct AppData {
    /// WS соединение установлено
    pub connected: bool,
    pub tick_count: u64,
    pub traces: u32,
    pub tension: u32,
    pub last_matched: u32,
    /// Домены из последнего BroadcastSnapshot
    pub domains: Vec<DomainSummary>,
    /// Токены для Space View (из последнего DomainDetail ответа)
    pub tokens: Vec<(u16, TokenSnapshot)>, // (domain_id, token)
    /// Вывод последней команды (CommandResult / Result)
    pub last_output: String,
    /// Последняя ошибка сервера или соединения
    pub last_error: Option<String>,
}
