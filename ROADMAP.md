# Axiom Roadmap

**Версия:** 25.0
**Дата:** 2026-04-09

---

## Текущее состояние

Axiom Sentinel V1.0 завершён. 900 тестов. В работе: закрытие технического долга.

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent
                                                (axiom-cli)
```

---

## Технический долг (в порядке выполнения)

---

### ~~Шаг 1: D-05 — data_dir дублируется~~ ✅

**Crates:** axiom-agent  
**Сложность:** малая

**Проблема:**  
`CliConfig.data_dir` и `AutoSaver.config.data_dir` — два отдельных поля.  
`AutoSaver` инициализируется из `CliConfig.data_dir` при создании, но при runtime-изменении пути они расходятся.

**Как исправить:**  
`CliChannel` является единственным владельцем `data_dir`. `AutoSaver` не должен хранить путь самостоятельно — он должен получать его от `CliChannel` при каждом сохранении.

**Чеклист:**
- [ ] Убрать `data_dir` из `PersistenceConfig` / `AutoSaver` как хранимое поле
- [ ] `AutoSaver::tick(&engine, data_dir)` и `force_save(&engine, data_dir)` — путь передаётся при вызове
- [ ] `:autosave on N` — `data_dir` берётся из `self.config.data_dir` в момент вызова
- [ ] `:load` — после загрузки `data_dir` не нужно синхронизировать отдельно
- [ ] Тесты: сохранение использует актуальный путь из `CliConfig`

---

### Шаг 2: D-02 — Event._pad: решение по семантике

**Crates:** axiom-core  
**Сложность:** малая

**Проблема:**  
`Event` имеет 2 байта анонимного выравнивания (`_pad: u16`) между `source_domain` и `snapshot_event_id`.

**Текущий layout:**
```
source_domain:      u16  // 2B — домен-источник
_pad:               u16  // 2B — только выравнивание
snapshot_event_id:  u32  // 4B
payload:          [u8;8] // 8B
```

**Решение (обсудить перед реализацией):**  
Кандидат — `target_domain: u16`. Event сейчас знает только источник, но не адресата. Это ограничивает маршрутизацию событий.  
Альтернатива — `event_subtype: u16` для уточнения типа внутри `event_type`.

**Чеклист:**
- [ ] Принять решение: `target_domain` vs `event_subtype`
- [ ] Переименовать `_pad` → выбранное поле в `crates/axiom-core/src/event.rs`
- [ ] Обновить все места создания `Event` (добавить значение поля)
- [ ] Обновить спецификацию `docs/spec/COM V1.1.md` или `Event-Driven V1.md`
- [ ] Тесты: новое поле корректно сохраняется и читается

---

### Шаг 3: D-03 — Token.reserved_phys: решение по семантике

**Crates:** axiom-core  
**Сложность:** малая

**Проблема:**  
`Token` имеет 2 байта анонимного выравнивания (`reserved_phys: u16`) между `target: [i16; 3]` и `valence: i8`.

**Решение (обсудить перед реализацией):**  
Кандидаты:
- `layer_id: u16` — номер фрактального уровня-владельца токена (для FractalChain routing)
- `hop_count: u16` — счётчик переходов между уровнями (для трассировки в multi-level системах)

**Чеклист:**
- [ ] Принять решение: `layer_id` vs `hop_count` (или оба? — нет, 2 байта = одно поле u16)
- [ ] Переименовать в `crates/axiom-core/src/token.rs:62`
- [ ] Обновить все места создания `Token` где нужно проставить значение
- [ ] Обновить спецификацию `docs/spec/Token V5.2.md`
- [ ] Тесты: новое поле корректно инициализируется и используется

---

### Шаг 4: D-01 — domain_id: u16 vs u32 — унификация

**Crates:** axiom-core, axiom-config, axiom-domain, axiom-runtime, axiom-agent  
**Сложность:** большая (затрагивает весь стек)

**Проблема:**  
Несогласованность типа `domain_id` по стеку. На каждой точке входа в Engine — неявный каст `token.domain_id as u32`.

| Место | Тип |
|---|---|
| `Token.domain_id` | `u16` |
| `Connection.domain_id` | `u16` |
| `DomainConfig.domain_id` | `u16` |
| `InjectTokenPayload.target_domain_id` | `u16` |
| `AshtiCore.inject_token(domain_id: u32)` | `u32` |
| `AshtiCore.index_of(domain_id: u32)` | `u32` |
| `AxiomEngine.token_count(domain_id: u32)` | `u32` |

**Решение:** Унифицировать в `u16` — engine API принимает `u16`. Структуры уже используют `u16`, значит рефакторинг затрагивает только сигнатуры методов Engine/AshtiCore, не layout Token/Connection.

**Чеклист:**
- [ ] `AshtiCore::inject_token(domain_id: u16)` — изменить сигнатуру
- [ ] `AshtiCore::index_of(domain_id: u16)` — изменить сигнатуру
- [ ] `AshtiCore::config_of(domain_id: u16)` — проверить и изменить если нужно
- [ ] `AxiomEngine::token_count(domain_id: u16)` — изменить сигнатуру
- [ ] `AxiomEngine::spawn_domain(domain_id: u16)` — проверить все engine-методы
- [ ] Убрать все `as u32` касты на границе Engine
- [ ] Проверить `axiom-agent`, `axiom-bench` — обновить вызовы
- [ ] Тесты: убедиться что все 900 тестов проходят после рефакторинга
- [ ] Обновить DEFERRED.md (D-01 закрыт)

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
