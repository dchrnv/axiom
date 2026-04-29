# AXIOM MODULE SPECIFICATION: FRAMEWEAVER V1.2

**Статус:** Актуальная спецификация (core)
**Версия:** 1.2.0
**Дата:** 2026-04-29
**Codename:** "Dream Weaver"
**Назначение:** Сборка и кристаллизация реляционных (синтаксических) узоров
**Crate:** `axiom-runtime` (модуль `over_domain/weavers/frame.rs`)
**Категория:** Weaver (Over-Domain Layer)
**Модель времени:** COM `event_id`
**Связанные спеки:** Over-Domain Layer V1.1, DREAM Phase V1.0, Connection V5.0, Token V5.2, Shell V3.0, Ashti_Core V2.1, GENOME V1.0, GUARDIAN V1.0, Memory Persistence V1.0

---

## 0. Изменения относительно V1.1

**V1.2 — разделение путей кристаллизации.** V1.1 описывала оба типа кристаллизации (EXPERIENCE и SUTRA) как часть цикла `on_tick`. Это противоречило семантике: промоция в SUTRA требует состояния DREAMING (онтологический инвариант GUARDIAN), а `on_tick` выполняется в WAKE.

### Что изменилось

1. **Шаги 4–5 `on_tick` удалены.** Цикл `on_tick` больше не оценивает Frame на промоцию и не отправляет `PromotionProposal`. Это устраняло ложный путь: предложение формировалось в WAKE, а GUARDIAN ветировал его (запись FRAME_ANCHOR в SUTRA вне DREAMING).

2. **Добавлен метод `dream_propose(ashti)`** — вызывается DreamCycle в начале DREAM-фазы (этап `tick_falling_asleep`). Возвращает `Vec<DreamProposal>` с предложениями промоции для всех Frame, соответствующих `PromotionRule`. Выполняется строго в переходе `FallingAsleep → Dreaming`, когда система уже вышла из WAKE.

3. **Инвариант GUARDIAN теперь замкнут.** `check_frame_anchor_sutra_write` вето выдаётся только вне DREAMING. С V1.2 FrameWeaver никогда не формирует промоционные команды в WAKE — оба слоя (Weaver и GUARDIAN) согласованы.

4. **`on_tick` остаётся путём WAKE-кристаллизации** (шаги 1–3: сканирование MAYA, обновление кандидатов, кристаллизация в EXPERIENCE). Этот путь не затронут.

### Что не изменилось

- Структура 8 синтаксических слоёв и `SemanticContributionTable` категория `0x08`.
- Правила кристаллизации (раздел 5.3) и правила промоции (раздел 5.4 — пороги, Codex approval).
- `PromotionRule`, `FrameCandidate`, `FrameWeaverConfig`, `FrameWeaverStats`.
- `build_promotion_commands()` — формирует `InjectFrameAnchorPayload` + `BondTokensPayload` для каждого участника. Реализация не изменилась.

---

## 4.3 Цикл работы (обновлён)

```
on_tick(tick, ashti, com):
    if tick % config.scan_interval_ticks != 0:
        return

    maya_state = ashti.peek_state(110)

    # 1. Сканировать MAYA на синтаксические узоры
    new_candidates = self.scan(maya_state)

    # 2. Обновить существующих кандидатов
    for candidate in self.candidates:
        if still_present_in_maya(candidate, maya_state):
            candidate.stability_count += 1
        else:
            self.candidates.remove(candidate.id)

    # 3. Кандидаты, достигшие стабильности → кристаллизация в EXPERIENCE (WAKE-путь)
    for candidate in self.candidates:
        if candidate.stability_count >= config.stability_threshold:
            # HeavyCrystallization proposal через DreamCycle (если система в Wake)
            # или прямая кристаллизация (V1.0: не реализована)
            ...

    # Шаги 4–5 (промоция в SUTRA) УДАЛЕНЫ.
    # Промоция происходит только через dream_propose() в DREAM-фазе.
```

```
dream_propose(ashti) -> Vec<DreamProposal>:
    # Вызывается ровно один раз при переходе FallingAsleep → Dreaming.
    # Состояние системы: dream_phase_state == FallingAsleep (переходит в Dreaming).
    # GUARDIAN разрешит SUTRA-запись, когда state станет Dreaming.

    experience_state = ashti.state(EXPERIENCE_INDEX)  # индекс 9 для level=1
    proposals = []

    for token in experience_state.tokens:
        if not (token.flags & TOKEN_FLAG_FRAME_ANCHOR):
            continue
        if qualifies_for_promotion(token, config.promotion_rules):
            proposals.push(DreamProposal {
                source:           WeaverId::Frame,
                kind:             DreamProposalKind::Promotion { anchor_id, source_domain, target_domain, rule_id },
                created_at_event: current_event_id,
            })

    return proposals
```

---

## 5.4 Поток промоции EXPERIENCE → SUTRA (обновлён)

```
1. dream_propose() вызывается DreamCycle при tick_falling_asleep
2. Для каждого Frame в EXPERIENCE проверяются PromotionRule (Genome + Config)
3. Если условия выполнены → DreamProposal::Promotion добавляется в DreamCycle queue
4. DreamCycle обрабатывает предложение в фазе Processing (state == Dreaming)
5. build_promotion_commands() формирует InjectFrameAnchorPayload + BondTokensPayload
6. GUARDIAN.check_frame_anchor_sutra_write() проверяет: state == Dreaming → OK
7. UCL-команды копируют Frame в SUTRA (новый anchor_id, TOKEN_FLAG_PROMOTED_FROM_EXPERIENCE)
8. Оригинал в EXPERIENCE сохраняется
```

**Ключевое отличие от V1.1:** шаг 1 происходит при засыпании (в переходе FallingAsleep→Dreaming), а не в `on_tick`. Это гарантирует, что к моменту выполнения команды система уже в DREAMING, и GUARDIAN пропускает SUTRA-запись.

---

## Статус V1.1

V1.1 **superseded** (заменена V1.2). Актуальная спека — данный документ.

Архивная копия: `docs/spec/Weaver/FrameWeaver_V1_1.md` — сохраняется как историческая версия.
