// AXIOM MODULE: COM V1.0 - Causal Order Model
//
// Модель времени как упорядоченности изменений.
// Заменяет глобальную временную ось на причинный порядок событий.
//
// Связанные спецификации:
// - docs/spec/COM V1.0.md (каноническая)
// - docs/spec/time/Time_Model_V1_0.md
// - docs/spec/time/Causal Frontier System V1.md

use crate::event::{Event, Timeline, EventType, EventPriority};

/// COM (Causal Order Model) - центральная структура управления причинным порядком
pub struct COM {
    timeline: Timeline,
    event_log: Vec<Event>,  // TODO: заменить на более эффективную структуру
    validation_enabled: bool,
}

impl COM {
    /// Создает новый COM с пустой временной линией
    pub fn new() -> Self {
        Self {
            timeline: Timeline::new(),
            event_log: Vec::new(),
            validation_enabled: true,
        }
    }

    /// Генерирует следующий event_id для домена
    ///
    /// Гарантирует монотонность согласно COM V1.0, инвариант 1
    pub fn next_event_id(&mut self, domain_id: u16) -> u64 {
        self.timeline.next_event_id(domain_id)
    }

    /// Получает текущий максимальный event_id
    pub fn current_event_id(&self) -> u64 {
        self.timeline.current_event_id
    }

    /// Валидация события согласно COM V1.0, раздел 10
    pub fn validate_event(&self, event: &Event) -> bool {
        if !self.validation_enabled {
            return true;
        }

        event.validate(&self.timeline)
    }

    /// Применяет событие к COM
    ///
    /// Валидирует событие и добавляет в лог если валидно.
    /// Возвращает true если событие было применено.
    pub fn apply_event(&mut self, event: Event) -> bool {
        if !self.validate_event(&event) {
            return false;
        }

        // Обновляем timeline если event_id больше текущего
        if event.event_id > self.timeline.current_event_id {
            self.timeline.current_event_id = event.event_id;
            self.timeline.domain_offsets[event.domain_id as usize] = event.event_id;
            self.timeline.total_events += 1;
        }

        self.event_log.push(event);
        true
    }

    /// Создает контрольную точку (checkpoint)
    ///
    /// Согласно COM V1.0, раздел 11.4
    pub fn create_checkpoint(&mut self) -> u64 {
        self.timeline.create_checkpoint()
    }

    /// Получает checkpoint_id
    pub fn checkpoint_id(&self) -> u64 {
        self.timeline.checkpoint_id
    }

    /// Получает общее количество событий
    pub fn total_events(&self) -> u64 {
        self.timeline.total_events
    }

    /// Получает события для домена
    pub fn events_for_domain(&self, domain_id: u16) -> Vec<&Event> {
        self.event_log
            .iter()
            .filter(|e| e.domain_id == domain_id)
            .collect()
    }

    /// Получает события в диапазоне event_id
    pub fn events_in_range(&self, start: u64, end: u64) -> Vec<&Event> {
        self.event_log
            .iter()
            .filter(|e| e.event_id >= start && e.event_id <= end)
            .collect()
    }

    /// Получает последнее событие для target_id
    pub fn last_event_for_target(&self, target_id: u32) -> Option<&Event> {
        self.event_log
            .iter()
            .rev()
            .find(|e| e.target_id == target_id)
    }

    /// Вычисляет причинный возраст (causal age) для сущности
    ///
    /// Согласно Time Model V1.0, раздел 2.2
    /// causal_age = current_event_id - entity.last_event_id
    pub fn compute_causal_age(&self, last_event_id: u64) -> u64 {
        if self.timeline.current_event_id >= last_event_id {
            self.timeline.current_event_id - last_event_id
        } else {
            0
        }
    }

    /// Отключает валидацию (для тестирования)
    #[cfg(test)]
    pub fn disable_validation(&mut self) {
        self.validation_enabled = false;
    }

    /// Очищает event log (сохраняет timeline)
    ///
    /// Используется после checkpoint для освобождения памяти
    pub fn clear_log(&mut self) {
        self.event_log.clear();
    }

    /// Получает количество событий в логе
    pub fn log_size(&self) -> usize {
        self.event_log.len()
    }
}

impl Default for COM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_creation() {
        let com = COM::new();
        assert_eq!(com.current_event_id(), 0);
        assert_eq!(com.total_events(), 0);
        assert_eq!(com.log_size(), 0);
    }

    #[test]
    fn test_next_event_id_monotonicity() {
        let mut com = COM::new();
        let domain_id = 1;

        let id1 = com.next_event_id(domain_id);
        let id2 = com.next_event_id(domain_id);
        let id3 = com.next_event_id(domain_id);

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(id3, 3);
        assert!(id1 < id2);
        assert!(id2 < id3);
    }

    #[test]
    fn test_domain_isolation() {
        let mut com = COM::new();

        let id_domain1 = com.next_event_id(1);
        let id_domain2 = com.next_event_id(2);
        let id_domain3 = com.next_event_id(1);

        assert_eq!(id_domain1, 1);
        assert_eq!(id_domain2, 2);
        assert_eq!(id_domain3, 3);
    }

    #[test]
    fn test_causal_age_computation() {
        let mut com = COM::new();

        let event_id_1 = com.next_event_id(1);  // 1
        let event_id_2 = com.next_event_id(1);  // 2
        let event_id_3 = com.next_event_id(1);  // 3

        assert_eq!(com.compute_causal_age(event_id_1), 2); // 3 - 1
        assert_eq!(com.compute_causal_age(event_id_2), 1); // 3 - 2
        assert_eq!(com.compute_causal_age(event_id_3), 0); // 3 - 3
    }

    #[test]
    fn test_apply_event() {
        let mut com = COM::new();
        let event_id = com.next_event_id(1);

        let event = Event::new(
            event_id,
            1, // domain_id
            EventType::TokenCreate,
            EventPriority::Normal,
            0x1234567890ABCDEF, // payload_hash
            100, // target_id
            0, // source_id
            0, // parent_event_id
        );

        let result = com.apply_event(event);
        assert!(result);
        assert_eq!(com.log_size(), 1);
        assert_eq!(com.total_events(), 1);
    }

    #[test]
    fn test_event_validation_parent_check() {
        let mut com = COM::new();

        let event_id_1 = com.next_event_id(1);  // 1
        let event_id_2 = com.next_event_id(1);  // 2

        // Событие 2 с parent_event_id = 1 (валидно)
        let valid_event = Event::new(
            event_id_2,
            1,
            EventType::TokenUpdate,
            EventPriority::Normal,
            0x1234567890ABCDEF,
            100,
            0,
            event_id_1, // parent < event_id
        );

        assert!(com.validate_event(&valid_event));

        // Событие 1 с parent_event_id = 2 (невалидно)
        let invalid_event = Event::new(
            event_id_1,
            1,
            EventType::TokenUpdate,
            EventPriority::Normal,
            0x1234567890ABCDEF,
            100,
            0,
            event_id_2, // parent >= event_id (невалидно)
        );

        assert!(!com.validate_event(&invalid_event));
    }

    #[test]
    fn test_checkpoint() {
        let mut com = COM::new();

        com.next_event_id(1);
        com.next_event_id(1);
        com.next_event_id(1);

        let checkpoint = com.create_checkpoint();
        assert_eq!(checkpoint, 3);
        assert_eq!(com.checkpoint_id(), 3);
    }

    #[test]
    fn test_events_for_domain() {
        let mut com = COM::new();
        com.disable_validation();

        let event1 = Event::new(1, 1, EventType::TokenCreate,
                               EventPriority::Normal, 0x1, 100, 0, 0);
        let event2 = Event::new(2, 2, EventType::TokenCreate,
                               EventPriority::Normal, 0x2, 101, 0, 0);
        let event3 = Event::new(3, 1, EventType::TokenUpdate,
                               EventPriority::Normal, 0x3, 100, 0, 1);

        com.apply_event(event1);
        com.apply_event(event2);
        com.apply_event(event3);

        let domain1_events = com.events_for_domain(1);
        assert_eq!(domain1_events.len(), 2);

        let domain2_events = com.events_for_domain(2);
        assert_eq!(domain2_events.len(), 1);
    }

    #[test]
    fn test_events_in_range() {
        let mut com = COM::new();
        com.disable_validation();

        for i in 1..=10 {
            let event = Event::new(i, 1, EventType::TokenCreate,
                                 EventPriority::Normal, i, 100, 0, 0);
            com.apply_event(event);
        }

        let events = com.events_in_range(3, 7);
        assert_eq!(events.len(), 5);
        assert_eq!(events[0].event_id, 3);
        assert_eq!(events[4].event_id, 7);
    }

    #[test]
    fn test_last_event_for_target() {
        let mut com = COM::new();
        com.disable_validation();

        let event1 = Event::new(1, 1, EventType::TokenCreate,
                               EventPriority::Normal, 0x1, 100, 0, 0);
        let event2 = Event::new(2, 1, EventType::TokenUpdate,
                               EventPriority::Normal, 0x2, 100, 0, 1);
        let event3 = Event::new(3, 1, EventType::TokenUpdate,
                               EventPriority::Normal, 0x3, 200, 0, 0);

        com.apply_event(event1);
        com.apply_event(event2);
        com.apply_event(event3);

        let last = com.last_event_for_target(100);
        assert!(last.is_some());
        assert_eq!(last.unwrap().event_id, 2);

        let last_200 = com.last_event_for_target(200);
        assert!(last_200.is_some());
        assert_eq!(last_200.unwrap().event_id, 3);
    }

    #[test]
    fn test_clear_log() {
        let mut com = COM::new();
        com.disable_validation();

        for i in 1..=5 {
            let event = Event::new(i, 1, EventType::TokenCreate,
                                 EventPriority::Normal, i, 100, 0, 0);
            com.apply_event(event);
        }

        assert_eq!(com.log_size(), 5);

        com.clear_log();

        assert_eq!(com.log_size(), 0);
        assert_eq!(com.current_event_id(), 5); // Timeline не сбрасывается
    }
}
