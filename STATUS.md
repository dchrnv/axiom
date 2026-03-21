# Axiom Status

**Версия:** v0.8.1
**Дата:** 2026-03-21

---

## ✅ v0.8.1 - SPACE ↔ Shell Integration (ЗАВЕРШЕНО)

**Выполнено (Phase 3.1):**
- **process_connection_event()**: Новая функция для пометки затронутых токенов как dirty при Connection событиях
- **Collision Integration**: При TokenCollision с существующей Connection - помечаем Shell dirty для обоих токенов
- **Connection Maintenance Integration**: При обработке Connection в frontier - автоматически вызываем process_connection_event()
- **End-to-End Flow**: Полный цикл: Connection event → Shell dirty → Heartbeat reconciliation → Shell update
- **Integration Tests**: 3 новых теста (process_connection_event, connection_maintenance, end-to-end flow)

**Поток данных:**
1. SPACE: столкновение → `TokenCollision` событие
2. Domain: обработка Connection в process_frontier() → вызов process_connection_event()
3. Shell: затронутые токены (source + target) помечаются dirty через mark_dirty()
4. Heartbeat: reconcile_shell_batch() пересчитывает только dirty токены
5. Cache: обновлённые профили сохраняются в DomainShellCache

**Тесты:** 336 pass (+3) ✅

**Файлы:**
- runtime/src/shell.rs (+process_connection_event, +28 строк)
- runtime/src/domain.rs (+2 integration points, +3 tests, +147 строк)
- runtime/src/lib.rs (экспорт process_connection_event)

**Коммиты:** d4760dd, c473d8f

---

## ✅ v0.8.0 - Shell V3.0 (ЗАВЕРШЕНО)

**Выполнено (Phases 2.1-2.10, skip 2.3, 2.9):**
- **ShellProfile**: 8-byte semantic profile [u8; 8] для 8 ортогональных слоёв восприятия (L1-L8)
- **DomainShellCache**: profiles + dirty_flags (BitVec) + generation counter для инкрементальных обновлений
- **SemanticContributionTable**: 256 категорий + HashMap overrides, двухуровневая иерархия
- **default_ashti_core()**: 7 базовых категорий (Structural, Semantic, Causal, Experiential, Social, Temporal, Motor)
- **compute_shell()**: алгоритм вычисления Shell с аккумулятором [f32; 8], нормализацией (max→255), округлением
- **Incremental Update**: mark_dirty() + update_dirty_shells() для пересчёта только изменённых токенов
- **Frontier Integration**: collect_affected_tokens() собирает source+target из Connection событий
- **Heartbeat Reconciliation**: reconcile_shell_batch() для drift detection в heartbeat батчах
- **Domain Integration**: shell_cache и semantic_table в Domain struct с инициализацией из DomainConfig
- **Validation**: 5 тестов инвариантов (детерминизм, домен-локальность, no COM events, cache coherence, zero-allocation)
- **Configuration**: Hardcoded (2 YAML конфигурации отложены в DEFERRED.md 3.5, 3.6)

**Тесты:** 333 pass (+48) ✅

**Файлы:**
- runtime/src/shell.rs (1032 строки, 48 тестов) - новый модуль
- runtime/src/domain.rs (+shell_cache + semantic_table fields)
- runtime/src/heartbeat.rs (+enable_shell_reconciliation flag)
- runtime/src/lib.rs (экспорт Shell API)
- runtime/Cargo.toml (+bitvec dependency)
- DEFERRED.md v3.5 (+2 YAML конфигурации отложены)

**Коммиты:** df155d4, f2a1221, 16a4e2f, b956b1a, c95a65e

---

## ✅ v0.7.0 - SPACE V6.0 (ЗАВЕРШЕНО)

**Выполнено (Phases 1.1-1.11):**
- **Spatial Hash Grid**: O(1) neighbor lookup, bucket-based linked lists
- **Distance Functions**: distance2(), distance2_to_anchor() - целочисленная арифметика
- **Gravity**: Linear & InverseSquare models, configurable gravity_scale_shift
- **Motion Physics**: velocity, friction, acceleration - saturating arithmetic
- **Spatial Events**: TokenMoved, TokenCollision, TokenEnteredCell
- **Collision Detection**: detect_collisions() через spatial hash
- **Domain Integration**: SpatialHashGrid в Domain, rebuild_frequency
- **Frontier Integration**: Collision detection в process_frontier, generate_collision()
- **Heartbeat Integration**: enable_spatial_collision flag, полный цикл Heartbeat → Spatial checks
- **Validation**: 3 тестa инвариантов (детерминизм, zero-alloc, cross-spec)
- **Configuration**: Hardcoded константы (YAML конфигурация отложена в DEFERRED.md 3.1)

**Тесты:** 285 pass (+105) ✅

**Файлы:**
- runtime/src/space.rs (1447 строк, 83 теста)
- runtime/src/domain.rs (+collision detection + 3 validation тестa)
- runtime/src/heartbeat.rs (+enable_spatial_collision flag)
- runtime/src/event_generator.rs (+generate_collision method)
- runtime/src/event.rs (+3 EventType)
- docs/spec/SPACE_V6_0.md, Shell_V3_0.md
- DEFERRED.md v3.4 (+YAML configuration отложена)

**Коммит:** 663ca07

---

## 📊 Модули

| Модуль | Spec | Runtime | Status |
|--------|------|---------|--------|
| Token | V5.2 | V5.2 | ✅ Complete |
| Connection | V5.0 | V5.0 | ✅ Complete |
| COM | V1.1 | V1.1 | ✅ Complete |
| UPO | V2.3 | V2.3 | ✅ Complete |
| Domain | V2.1 | V2.1 | ✅ Complete |
| EventGenerator | V1 | V1 | ✅ Complete |
| CausalFrontier | V1 | V1 | ✅ Complete |
| Heartbeat | V2.0 | V2.0 | ✅ Complete |
| Experience | V1 | V1 | ✅ Complete |
| Arbiter | V2.1 | V2.1 | ✅ Complete |
| SPACE | V6.0 | V6.0 | ✅ Complete |
| Shell | V3.0 | V3.0 | ✅ Complete |

---

**Последнее обновление:** 2026-03-21
