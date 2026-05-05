# AXIOM Workstation V1.0 — Errata

**Дата создания:** 2026-05-05
**Назначение:** фиксация расхождений спецификации V1.0 с реальной реализацией, обнаруженных в процессе Этапов 0–11.

---

## E1. `iced::subscription::channel` отсутствует в iced 0.13

**Этап:** 3

Спека предполагала `iced::subscription::channel(size, FnOnce)` для создания WebSocket-подписки. В iced 0.13 этот API не существует.

**Решение в реализации:**
`iced::stream::channel(size, FnOnce)` + `iced::Subscription::run_with_id(id, stream)`.

**Статус:** закрыто. Реализация консистентна.

---

## E2. `iced::application` не поддерживает multi-window

**Этап:** 4

Спека и начальный код использовали `iced::application`. Multi-window (разные view по `window::Id`) требует `iced::daemon`.

**Решение в реализации:**
Переход на `iced::daemon(title, update, view)` с сигнатурой `view(&self, id: window::Id) -> Element`.

**Статус:** закрыто.

---

## E3. `Padding` не принимает `[i32; 4]`

**Этап:** 4, 5

Спека и ранние фрагменты кода использовали `Padding::from([top, right, bottom, left])` с `i32`. В iced 0.13 поддерживается только `Padding { top, right, bottom, left }` (поля `f32`) или `f32`/`u16` одним значением.

**Решение в реализации:**
Явные поля: `Padding { top: T, right: R, bottom: B, left: L }`.

**Статус:** закрыто.

---

## E4. `on_key_press` принимает `fn` pointer, не замыкание

**Этап:** 9

`iced::keyboard::on_key_press` требует указатель на функцию `fn(Key, Modifiers) -> Option<Message>` — хешируемый тип. Замыкание не подходит (не реализует `Hash`).

**Решение в реализации:**
Вынесена module-level функция `fn keyboard_shortcut(key: Key, mods: Modifiers) -> Option<Message>`.

**Статус:** закрыто.

---

## E5. WebSocket subscription не перезапускается при том же строковом ID

**Этап:** 9 (fix WS5-TD-01)

`Subscription::run_with_id(address.clone(), stream)` — если адрес не менялся, iced считает подписку той же и не перезапускает. При смене адреса через ConfigApply subscription оставалась на старом адресе.

**Решение в реализации:**
ID изменён на кортеж `(address.clone(), key)` + поле `subscription_key: u64` в `WorkstationApp`, инкрементируется при ConfigApply для workstation.connection.

**Статус:** закрыто.

---

## E6. `Element` в iced 0.13 не имеет метода `.width()`

**Этап:** 7

Попытка вызвать `.width(N)` на `Element` не компилируется. Метод принадлежит конкретным widget-типам (`text`, `button`, и т.д.), но не обобщённому `Element`.

**Решение в реализации:**
`container(elem).width(N)` — оборачивание в `Container` с нужной шириной.

**Статус:** закрыто.

---

## E7. Явная аннотация типа `Element` при условном `if`

**Этап:** 8

В условных ветках `if condition { text("a") } else { text("b") }` компилятор не может вывести параметр `Theme` для `Element<'a, Message>`.

**Решение в реализации:**
Явная аннотация: `let widget: Element<'a, Message> = if ... { ... } else { ... };`

**Статус:** закрыто.

---

## E8. `DomainSnapshot` не содержит индивидуальных позиций токенов

**Этап:** 10 (WS10-TD-01)

Спека Live Field предполагала рендер реальных позиций токенов из `DomainSnapshot`. В протоколе `DomainSnapshot` содержит только агрегированные статистики домена — без `Vec<TokenSnapshot>`.

**Решение в реализации:**
Процедурная визуализация: детерминированный LCG по `(domain_id, index)` генерирует позиции. Реальные данные — отложено в WS10-TD-01 (требует расширения протокола).

**Статус:** открыто (WS10-TD-01).

---

## E9. `canvas` и `tokio` требуют явных features в iced

**Этап:** 4

`iced::widget::canvas` и `iced::time::every` не доступны без явного указания `features = ["canvas", "tokio"]` в `Cargo.toml`.

**Решение в реализации:**
`iced = { version = "0.13", features = ["canvas", "tokio"] }` в axiom-workstation/Cargo.toml.

**Статус:** закрыто.

---

## Resolution Summary

| #  | Этап | Статус   | Краткое описание |
|----|------|----------|-----------------|
| E1 | 3    | Закрыто  | `subscription::channel` → `stream::channel + run_with_id` |
| E2 | 4    | Закрыто  | `application` → `daemon` для multi-window |
| E3 | 4,5  | Закрыто  | `Padding` — struct literal вместо массива |
| E4 | 9    | Закрыто  | `on_key_press` — module-level fn вместо замыкания |
| E5 | 9    | Закрыто  | subscription ID кортеж `(address, key)` |
| E6 | 7    | Закрыто  | `container(elem).width(N)` вместо `.width()` |
| E7 | 8    | Закрыто  | явная аннотация `Element<'a, Message>` в `if` |
| E8 | 10   | Открыто  | `DomainSnapshot` без токенов — процедурный LCG (WS10-TD-01) |
| E9 | 4    | Закрыто  | iced features: canvas, tokio |
