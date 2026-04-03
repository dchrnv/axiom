# Axiom Roadmap

**Версия:** 20.0
**Дата:** 2026-04-02

---

## Следующие задачи (из DEFERRED.md)

### Приоритет 1 — tick_count в Snapshot (D-06)

**Сложность:** Минимальная. **Риск:** Низкий.

Файлы: `crates/axiom-runtime/src/snapshot.rs`, `crates/axiom-runtime/src/engine.rs`

```rust
// snapshot.rs — добавить поле
pub struct EngineSnapshot {
    pub domains: Vec<DomainSnapshot>,
    pub com_next_id: u64,
    pub tick_count: u64,   // NEW
    pub created_at: u64,
}

// engine.rs — snapshot()
EngineSnapshot {
    com_next_id: self.com_next_id,
    tick_count: self.tick_count,   // NEW
    ...
}

// engine.rs — restore_from()
engine.com_next_id = snapshot.com_next_id;
engine.tick_count  = snapshot.tick_count;  // NEW
```

Тесты: `test_tick_count_saved_in_snapshot`, `test_tick_count_restored`.

---

### Приоритет 2 — AshtiCore::reconcile_all() (D-09)

**Сложность:** Средняя. **Риск:** Низкий.

Файлы: `crates/axiom-domain/src/ashti_core.rs`, `crates/axiom-runtime/src/engine.rs`

Три задачи в одном методе:
1. Форсировать `rebuild_spatial_grid()` для доменов где `should_rebuild_spatial_grid() == true`
2. Удалить связи, чьи `source_id` / `target_id` ссылаются на несуществующие токены
3. Исправить `token.domain_id` если токен оказался не в своём домене

После реализации: вызов в `handle_tick_forward` по `reconcile_interval` (уже подключён в TickSchedule, поле есть).

Тесты: `test_reconcile_prunes_orphan_connections`, `test_reconcile_rebuilds_spatial_grid`, `test_reconcile_fixes_domain_id`.

---

### Приоритет 3 — compare_tokens tolerances в DomainConfig (D-04 + D-05)

**Сложность:** Средняя. **Риск:** Низкий (regression тест есть).

Использовать `reserved_id: u64` (первые 6 байт из 8 свободны):

```rust
// domain_config.rs — заменить reserved_id: u64 на:
pub token_compare_temp_tolerance:    i16,  // default: 10
pub token_compare_mass_tolerance:    i16,  // default: 5
pub token_compare_valence_tolerance: i16,  // default: 2
pub _reserved_id_tail:               u16,  // 2B остаток
```

В `compare_tokens()` (arbiter/lib.rs) читать из конфига домена вместо глобальных констант.
Глобальные константы `TOKEN_COMPARE_*_TOLERANCE` оставить как fallback defaults.

Тесты: regression на текущее поведение + `tolerance=0` → только точное совпадение.

---

### Приоритет 4 — UCL мёртвые опкоды (D-07)

**Сложность:** Низкая. **Риск:** Нулевой.

Файл: `crates/axiom-runtime/src/engine.rs:process_command`

Добавить явные ветки для 9 нереализованных опкодов → `Success, events=0` (no-op):

```rust
OpCode::LockMembrane
| OpCode::ReshapeDomain
| OpCode::ApplyForce
| OpCode::AnnihilateToken
| OpCode::BondTokens
| OpCode::SplitToken
| OpCode::ChangeTemperature
| OpCode::ApplyGravity
| OpCode::PhaseTransition =>
    make_result(cmd.command_id, CommandStatus::Success, error_codes::OK, 0),
```

Тест: каждый из 9 опкодов → `is_success()`.

---

### Приоритет 5 — StructuralRole vs UCL factory_preset (D-08)

**Сложность:** Средняя. **Риск:** Средний (API boundary).

Файл: `crates/axiom-ucl/src/lib.rs`, `crates/axiom-runtime/src/engine.rs:handle_spawn_domain`

Добавить явный маппинг `factory_preset → StructuralRole` в `handle_spawn_domain`:

```rust
fn ucl_preset_to_structural_role(preset: u8) -> u8 {
    match preset {
        0 => 8,  // Void
        1 => 0,  // Sutra
        n => n,  // остальные совпадают
    }
}
```

Тест: `factory_preset=0` → роль Void (8), `factory_preset=1` → роль Sutra (0).

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
