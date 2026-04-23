# Axiom Roadmap

**Версия:** 36.0  
**Дата:** 2026-04-23

---

## Текущее состояние

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent
                                                (axiom-cli)
```

**Фазы 1–3 FrameWeaver V1.1 завершены.** 991 тестов, 0 failures.  
**Over-Domain Layer** создан: traits `OverDomainComponent` + `Weaver`, `FrameWeaver` в `over_domain/weavers/frame.rs`.  
**Онтология:** SUTRA / EXPERIENCE / MAYA. Frame живёт в EXPERIENCE (STATE_ACTIVE), промоция в SUTRA через CODEX.

---

## Активная задача: FrameWeaver V1.1 — завершение

**Спека:** [FrameWeaver_V1_1.md](docs/spec/Weaver/FrameWeaver_V1_1.md)  
**Архитектура:** [Over_Domain_Layer_V1_1.md](docs/spec/Weaver/Over_Domain_Layer_V1_1.md)

### Фаза 4 — Интеграция

- [ ] Создать `FrameWeaver` в `AxiomEngine::new()` — после GUARDIAN
- [ ] Подключить к `TickSchedule` через `weaver_scan_intervals` и `weaver_promotion_intervals`
- [ ] Добавить `FrameWeaverStats` в `BroadcastSnapshot` (feature `adapters`):
  `frames_in_experience`, `frame_reactivations`, `promotions_proposed`, `promotions_completed`
- [ ] Загрузка `FrameWeaverConfig` из Schema Configuration
- [ ] Добавить `ModuleId::FrameWeaver` в GENOME с правами Read на MAYA/SUTRA/EXPERIENCE,
  Write на EXPERIENCE; Write на SUTRA только при промоции с CODEX-санкцией

### Фаза 5 — Тесты и документация

- [ ] Тест: `scan` находит синтаксические связи в MAYA
- [ ] Тест: стабильность кандидата нарастает, нестабильный удаляется
- [ ] Тест: `build_crystallization_commands` генерирует UCL с target domain_id=109 (EXPERIENCE)
- [ ] Тест: Frame anchor в EXPERIENCE — STATE_ACTIVE (не STATE_LOCKED)
- [ ] Тест: ReinforceFrame — повторный паттерн усиливает существующий Frame, не создаёт дубль
  (проверка по lineage_hash)
- [ ] Тест: `UnfoldFrame` разворачивает Frame в целевой домен
- [ ] Тест: обработка цикла — Allow по умолчанию (циклы допустимы в EXPERIENCE)
- [ ] Тест: промоция EXPERIENCE → SUTRA: Frame удовлетворяет PromotionRule →
  SUTRA получает токен STATE_LOCKED с TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE
- [ ] Тест: GUARDIAN накладывает veto на нарушающий Frame
- [ ] Обновить BLUEPRINT.md: статус FrameWeaver → ✅
- [ ] Обновить STATUS.md: этап FrameWeaver V1.1 ✅

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
