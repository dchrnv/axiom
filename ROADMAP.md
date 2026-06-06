# Axiom Roadmap

**Версия:** 76.0
**Дата:** 2026-06-04

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent (axiom-cli)
axiom-corpus                                        ↑
                                               axiom-broadcasting
```

**1714 тестов, 0 failures.**
PRIM-TD-03 Subsystem Gravity завершён (2026-06-04).
Sensorium V1.0, Waves V1.0, Cross-Modal Binding pipeline замкнуты (2026-06-03).

---

## Активные задачи

---

### Shell-TD-02 — resonance_search shell bonus

**Суть:** `resonance_search` в `Experience` оценивает сходство токена по temperature/mass/valence/position
но игнорирует shell-профиль `[u8;8]`. Shell несёт семантическую принадлежность к слоям L1–L8.
Токен с похожим shell-профилем должен получать бонус к score.

**Контекст:**
- `pattern_similarity()` — `crates/axiom-arbiter/src/experience.rs:505`. Нет shell.
- `shell_registry: HashMap<u32, [u8;8]>` — живёт в `AxiomEngine` (engine.rs:236), передаётся в FrameWeaver и ContextRecognizer, но НЕ в Experience/Arbiter.
- Shell-данные уже заполняются при `inject_anchor_tokens` (шаг 4).

**Шаги:**

1. **`shell_registry` в `Experience`** (`crates/axiom-arbiter/src/experience.rs`):
   ```rust
   shell_registry: HashMap<u32, [u8;8]>,  // sutra_id → shell profile
   ```
   Метод `set_shell_registry(&mut self, registry: HashMap<u32, [u8;8]>)`.

2. **Пробросить через Arbiter → AshtiCore:**
   - `Arbiter::set_shell_registry()` → делегирует в `self.experience.set_shell_registry()`
   - `AshtiCore::set_shell_registry()` → делегирует в `self.arbiter.set_shell_registry()`

3. **Вызвать в `inject_anchor_tokens`** (`engine.rs`, после шага 4):
   ```rust
   self.ashti.set_shell_registry(self.shell_registry.clone());
   ```

4. **Shell cosine similarity** (`crates/axiom-arbiter/src/experience.rs`):
   ```rust
   fn shell_cosine(a: &[u8;8], b: &[u8;8]) -> f32 {
       // dot / (|a| * |b|), NaN → 0.5 (нейтраль если профиль нулевой)
   }
   ```

5. **Бонус в `pattern_similarity`:**
   Сигнатура: `fn pattern_similarity(a: &Token, b: &Token, registry: &HashMap<u32,[u8;8]>) -> f32`
   ```
   base_score = (temp_diff + mass_diff + val_diff + pos_diff) * 0.25
   shell_sim  = shell_cosine(registry[a.sutra_id], registry[b.sutra_id])
                // если оба sutra_id есть в registry, иначе 0.5 (нейтраль)
   final      = base_score * (0.85 + 0.15 * shell_sim)
   ```
   Shell — 15% модификатор, не доминирует. Токены без shell в registry получают нейтральный вес.

6. **Обновить все вызовы `pattern_similarity`** в experience.rs (4 места) — передать `&self.shell_registry`.

7. **Тесты** (~5):
   - `test_shell_bonus_improves_similar_profile` — одинаковые shell → score выше чем без shell
   - `test_shell_penalty_for_different_profile` — разные shell → score ниже нейтрального
   - `test_no_shell_in_registry_neutral` — sutra_id не в registry → score = base (нейтраль 0.5)
   - `test_shell_cosine_zero_profile` — нулевой shell → 0.5 (не NaN)
   - `test_set_shell_registry_propagates` — после set_shell_registry поиск использует профили

---

### SEN-TD-01 — Sensorium V2.0: поглощение BroadcastSnapshot

**Суть:** сейчас два независимых пульса наружу:
1. `BroadcastSnapshot` → `BroadcastHandle` → WebSocket → Workstation/OBS/tray *(старый)*
2. `Sensorium.current_state: SensoriumState` → нигде не публикуется *(новый)*

Спека §7: "level 0 = переоформленный TickSnapshot — не создавать заново."
V2.0 = Sensorium становится единственным источником. `BroadcastSnapshot` удаляется.

**Текущий размер `BroadcastSnapshot`:** tick_count, com_next_id, trace_count, tension_count,
domain_summaries, frame_weaver_stats, dream_phase, last_crystallization_tick,
guardian_vetoes_since_wake, last_dream_summary, cross_modal_candidates, cross_modal_bonds.

**Текущий размер `SensoriumState`:** collected_at_tick, causal_time, active_subsystems,
dominant_subsystem, activity_signature, dominant_octant, corpus_callosum_active,
active_dilemma_count, active_dilemmas, has_pending_crystallization, candidates_count,
avg_shell_similarity, emergent_candidates, dream_phase_raw, fatigued_subsystems,
composite_suspect_count, cross_modal_bonds, internal_dominance_factor, active_impulse_count,
impulse_sources.

**Фазы реализации:**

#### Фаза A — Расширить SensoriumState до уровня BroadcastSnapshot
Файл: `crates/axiom-runtime/src/over_domain/sensorium/state.rs`

Добавить поля (level 0 = Pulse, собираются каждый тик):
```rust
pub tick_count: u64,
pub com_next_id: u64,
pub trace_count: usize,
pub tension_count: usize,
pub domain_summaries: Vec<SensoriumDomainSummary>,  // упрощённый DomainSummary
pub frame_weaver_stats: Option<SensoriumFrameStats>,
pub dream_phase_snapshot: Option<SensoriumDreamSnapshot>,
pub last_crystallization_tick: u64,
pub guardian_vetoes_since_wake: u64,
pub cross_modal_candidates: usize,
```
Заполнять в `collect_pulse()` из `SensoriumView` (туда добавить нужные ссылки).

#### Фаза B — Публиковать SensoriumState через BroadcastHandle
Файл: `crates/axiom-broadcasting/src/lib.rs` + `crates/axiom-node/src/tick.rs`

- `BroadcastHandle` получает метод `publish_sensorium(state: SensoriumState)`
- В `tick.rs`: после `engine.sensorium.collect()` → `broadcast.publish_sensorium(state)`
- WebSocket-клиенты получают `SensoriumState` вместо `BroadcastSnapshot`

#### Фаза C — Migrage axiom-web
Файл: `tools/axiom-web/src/store/` + `tools/axiom-web/src/ws/protocol.ts`

Обновить TypeScript-типы под `SensoriumState`. Workstation переключается с `BroadcastSnapshot` на новый формат. Все 8 табов должны работать.

#### Фаза D — Migrate axiom-observe
Файл: `crates/axiom-observe/src/`

OBS-runner получает SensoriumState вместо TickSnapshot. Поля `TickSnapshot V6` → поля из `SensoriumState`.

#### Фаза E — Migrate axiom-tray
Файл: `tools/axiom-tray/src/`

Трей читает `/metrics` (Prometheus) — не трогать. Если читает WS → обновить.

#### Фаза F — Удалить BroadcastSnapshot
- Удалить `snapshot_for_broadcast()` из engine.rs
- Удалить `crates/axiom-runtime/src/broadcast.rs` (или свести к re-export)
- Удалить `pub mod broadcast` из lib.rs
- Удалить feature "adapters" если он только для BroadcastSnapshot

**Тесты:**
- `test_sensorium_level0_has_all_broadcast_fields` — все поля BroadcastSnapshot есть в SensoriumState
- `test_publish_sensorium_reaches_subscriber` — после publish подписчик получает state
- Регрессионные тесты OBS: прогон корпуса должен дать те же метрики

**Порядок:** A → B → C → D → E → F. Каждая фаза компилируется и тестируется независимо.
Фазы C–E можно делать параллельно после B.

---

## Не в активном плане

- **BRD-TD-06** — Pong timeout test: требует raw TCP клиент без WS framing.
- **OBS-MON-01/02** — Мониторинг трасс и activity dynamics. См. DEFERRED.md.
- **COMP-01** — Vital Signs окно (Companion). См. DEFERRED.md.
- **V7-D: SubsystemExport/Import** — обмен подсистемами между инстансами.
- **V8** — Axiogenesis through Dilemmas. После 6+ месяцев реальной работы.
- **V9** — Active NeuralAdvisor (нейронные модели). После накопленной истории.

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
