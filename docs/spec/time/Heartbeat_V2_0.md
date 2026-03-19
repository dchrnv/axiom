# AXIOM MODULE SPECIFICATION: Heartbeat V2.0

**Статус:** Дополнение к Event-Driven V1 и Causal Frontier V1  
**Версия:** 2.0.0  
**Дата:** 2026-03-19  
**Назначение:** Периодическая активация фоновых процессов без нарушения причинной модели  
**Модель времени:** COM `event_id` (причинный порядок, u64)  
**Связанные спеки:** Time Model V1.0, COM V1.0, Event-Driven V1, Causal Frontier V1, DomainConfig V2.0, Token V5.2, Connection V5.0

---

## 1. Назначение

**Heartbeat** — специальное COM-событие, генерируемое с заданной периодичностью по числу событий. Heartbeat не является источником логики. Он служит триггером для активации фоновых процессов через механизм Causal Frontier.

Heartbeat решает проблему: некоторые процессы (затухание, гравитация, обслуживание связей) не привязаны к конкретному событию, но требуют периодической проверки. Heartbeat выражает факт: "причинная глубина системы увеличилась на N шагов".

---

## 2. Основной принцип

Heartbeat — это **легитимный тип причинности** (Time Model V1.0, Правило 3).

"Прошло N событий" является достаточной причиной для генерации нового события, так же как "токены столкнулись" или "связь разорвалась". Heartbeat не возвращает время в систему.

---

## 3. Генерация

### 3.1 Единственный режим: по числу событий

Heartbeat генерируется строго по счётчику событий. Это гарантирует полный детерминизм.

```rust
pub struct HeartbeatGenerator {
    interval: u32,                    // Количество событий между пульсами
    events_since_last_heartbeat: u32, // Счётчик
    pulse_number: u64,                // Монотонный номер пульса
}

impl HeartbeatGenerator {
    pub fn on_event(&mut self) -> Option<HeartbeatEvent> {
        self.events_since_last_heartbeat += 1;
        
        if self.events_since_last_heartbeat >= self.interval {
            self.events_since_last_heartbeat = 0;
            self.pulse_number += 1;
            
            Some(HeartbeatEvent {
                pulse_number: self.pulse_number,
            })
        } else {
            None
        }
    }
}
```

Heartbeat получает следующий `event_id` из COM и обрабатывается как обычное событие в причинном порядке.

### 3.2 Реальное время — только в адаптере

Если внешнему потребителю нужна синхронизация с wall-clock (LLM, сенсоры, UI), адаптер может генерировать UCL-команды с заданной периодичностью в миллисекундах. Эти команды проходят через стандартный путь:

```
Адаптер (таймер 100мс) → UCL Command → Ядро → COM event_id
```

Ядро не знает, что команда пришла по таймеру. Оно просто обрабатывает команду и присваивает `event_id`. Детерминизм внутренней логики не нарушается.

---

## 4. Структура события

```rust
#[repr(C)]
pub struct HeartbeatEvent {
    pub pulse_number: u64,  // Монотонный номер пульса (уникален в пределах домена)
}
```

HeartbeatEvent упаковывается в стандартный COM Event:

```rust
Event {
    event_id: <следующий из COM>,
    domain_id: <ID домена>,
    event_type: EventType::Heartbeat,  // Новый тип в COM
    payload_hash: hash(pulse_number),
    ...
}
```

---

## 5. Опциональная привязка событий к пульсу

Для удобства анализа и отладки в COM Event может быть добавлено поле `pulse_id`:

```rust
pub struct Event {
    // ... существующие поля COM V1.0 ...
    pub pulse_id: u64,  // Номер текущего пульса при создании события
}
```

Это позволяет:

- Фильтровать события по пульсам
- Воспроизводить симуляцию по пульсам (все события с `pulse_id <= N`)
- Анализировать, что произошло за каждый интервал

Поле опционально и включается через конфигурацию.

---

## 6. Обработка: что делает Heartbeat

Обработчик Heartbeat **не выполняет тяжёлых операций**. Он только добавляет сущности в Causal Frontier для последующей ленивой обработки.

```rust
fn handle_heartbeat(
    domain_state: &DomainState, 
    frontier: &mut CausalFrontier,
    heartbeat: &HeartbeatEvent, 
    config: &HeartbeatConfig,
) {
    let total_tokens = domain_state.token_count;
    
    // Добавляем batch_size токенов в frontier для обслуживания
    for i in 0..config.batch_size {
        let token_idx = ((heartbeat.pulse_number as usize) * config.batch_size + i) 
                        % total_tokens;
        frontier.push_token(token_idx);
    }
    
    // Аналогично для связей, если включено
    if config.enable_connection_maintenance {
        let total_connections = domain_state.connection_count;
        for i in 0..config.connection_batch_size {
            let conn_idx = ((heartbeat.pulse_number as usize) * config.connection_batch_size + i)
                           % total_connections;
            frontier.push_connection(conn_idx);
        }
    }
}
```

Frontier уже содержит механизм дедупликации. Повторное добавление одной сущности не приводит к повторной обработке.

### 6.1 Процессы, активируемые через Frontier

Когда сущность попадает в Frontier через Heartbeat, стандартный цикл обработки Frontier проверяет её состояние и при необходимости генерирует события:

**Затухание (Decay)**  
Вычисляется причинный возраст токена. Если `current_event_id - token.last_event_id` превышает порог, генерируется событие затухания. Используются параметры `decay_rate` из DomainConfig.

**Гравитация**  
Пересчитывается гравитационное влияние на токен. Если позиция должна измениться, генерируется событие движения. Используется `gravity_strength` из DomainConfig.

**Обслуживание связей**  
Проверяется стресс связи. Если `current_stress` превышает порог разрыва, генерируется событие разрыва или ослабления.

**Термодинамика**  
Температура токена адаптируется к температуре поля домена через причинный возраст.

### 6.2 Детерминированный выбор сущностей

Выбор сущностей для обработки строго детерминирован:

```
token_index = (pulse_number * batch_size + offset) % total_tokens
```

За достаточное количество пульсов каждая сущность будет обработана. Нагрузка распределена равномерно.

---

## 7. Конфигурация

Параметры Heartbeat задаются в DomainConfig. Разные домены могут иметь разные настройки.

```rust
pub struct HeartbeatConfig {
    pub interval: u32,                    // Количество событий между пульсами
    pub batch_size: usize,                // Токенов в frontier за пульс
    pub connection_batch_size: usize,     // Связей в frontier за пульс
    pub enable_decay: bool,               // Активировать затухание
    pub enable_gravity: bool,             // Активировать гравитацию
    pub enable_connection_maintenance: bool, // Активировать обслуживание связей
    pub enable_thermodynamics: bool,      // Активировать термодинамику
    pub attach_pulse_id: bool,            // Добавлять pulse_id к событиям
}
```

### 7.1 Примеры конфигурации

Для слабого оборудования (минимальная нагрузка):

```yaml
heartbeat:
  interval: 10000        # Редкие пульсы
  batch_size: 1           # Один токен за пульс
  connection_batch_size: 1
  enable_decay: true
  enable_gravity: false   # Отключена для экономии
  enable_connection_maintenance: false
  enable_thermodynamics: false
```

Для среднего оборудования:

```yaml
heartbeat:
  interval: 1024
  batch_size: 10
  connection_batch_size: 5
  enable_decay: true
  enable_gravity: true
  enable_connection_maintenance: true
  enable_thermodynamics: true
```

Для мощного сервера:

```yaml
heartbeat:
  interval: 256
  batch_size: 50
  connection_batch_size: 25
  enable_decay: true
  enable_gravity: true
  enable_connection_maintenance: true
  enable_thermodynamics: true
```

Конкретные значения подбираются при профилировании. В будущем могут быть оформлены как именованные пресеты.

---

## 8. Интеграция с Causal Frontier

Heartbeat является **поставщиком работы** для Causal Frontier, а не исполнителем. Весь фактический пересчёт (decay, gravity, stress) происходит внутри стандартного цикла обработки Frontier:

```
HeartbeatEvent
    ↓
handle_heartbeat() — добавляет сущности в frontier
    ↓
Frontier.pop() — стандартный цикл
    ↓
evaluate_local_rules(entity) — проверка причинного возраста, стресса
    ↓
generate_event() — DecayApplied, GravityUpdated, ConnectionWeakened
    ↓
apply_event() — обновление состояния
    ↓
COM event_id — причинный порядок сохранён
```

Таким образом Heartbeat не нарушает архитектуру Event-Driven модели. Он лишь обеспечивает, что сущности, не затронутые пользовательскими событиями, всё равно периодически проверяются.

---

## 9. Доменная изоляция

Каждый домен имеет собственный HeartbeatGenerator и собственный HeartbeatConfig. Домены генерируют Heartbeat независимо друг от друга.

Heartbeat одного домена **не влияет** на другие домены. Междоменное взаимодействие происходит только через COM.

---

## 10. Инварианты

1. **Детерминизм**: При одинаковом `interval` и одинаковой последовательности событий, Heartbeat генерируется в одних и тех же точках причинного порядка.
2. **COM совместимость**: HeartbeatEvent получает `event_id` из COM и занимает позицию в причинном порядке.
3. **Локальность**: Обработчик Heartbeat только добавляет сущности в Frontier. Вся дальнейшая логика — в стандартном цикле Frontier.
4. **Ограниченная нагрузка**: Количество сущностей, добавляемых в Frontier за один пульс, жёстко ограничено `batch_size`.
5. **Полное покрытие**: За `ceil(total_entities / batch_size)` пульсов каждая сущность будет обработана хотя бы один раз.

---

## 11. Состояние Idle

Если в системе нет внешних событий и Frontier пуст, Heartbeat **не генерируется** (нет событий → счётчик не растёт → пульс не срабатывает).

Система входит в состояние causal idle (см. Causal Frontier V1, раздел 8). CPU не используется. Система ждёт внешнего события.

Это ключевое свойство для работы на слабом оборудовании: если ничего не происходит, система потребляет ноль ресурсов.

---

## 12. История изменений

- **V2.0**: Полная переработка. Терминология приведена к Axiom (Token, Connection, Domain). Убраны shells/clusters. Убраны неопределённые профили (Lamp, Edge, Core). Привязка к DomainConfig. Интеграция с Causal Frontier. Единственный режим: по числу событий. Реальное время вынесено в адаптер.
- **V1.0**: Первая версия с двумя режимами генерации и профилями оборудования.
