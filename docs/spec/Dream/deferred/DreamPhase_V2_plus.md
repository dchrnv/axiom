# Deferred: вне DREAM Phase V1.0

Эти задачи не входят в V1.0 реализацию. См. также раздел 10 спецификации `DREAM_Phase_V1_0.md`.

---

## V2.0 кандидаты

**Recombination этап.**  
Полная реализация описана в спеке V1.0, раздел 4.4.1 (фиксация замысла). В V1.0 — заглушка `do_recombination_stub()`. Требует отдельной спеки `DREAM_Phase_V2_0_Recombination.md`.  
Суть: выгрузка выборки токенов из EXPERIENCE в DREAM(107), прогон физики zero-G high-T среды, поиск эмерджентных паттернов, возврат находок с флагом `TOKEN_FLAG_DREAM_ORIGIN`.

**Curiosity impulses.**  
Генерация в DREAM-фазе вопросов/гипотез, которые в WAKE приводят к probe-поведению. Не спроектировано.

**Skill condensation.**  
Более общая категория, чем кристаллизация Frame. Включает CodexAction proposals, выявление ключевых правил. `DreamProposalKind::SkillCondensation` зарезервирован в типах V1.0.

**Адаптивные параметры DreamScheduler.**  
`fatigue_threshold` и `idle_threshold_ticks` подстраиваются под ритм системы автоматически. В V1.0 — фиксированные дефолты.

**Активное использование DreamReport-токенов.**  
В V1.0 DreamReport записывается в EXPERIENCE но не анализируется. V2.0: чтение истории снов, паттерны одобрения/отклонения proposals, статистика качества снов.

**`DreamProposalKind::CodexProposal`.**  
Унификация с `engine.dream_propose()` (существующий метод, предлагающий в CODEX паттерны с weight ≥ 0.9 и success_count ≥ 5). Требует отдельного проектирования.

---

## Deferred (без сроков)

**Межсистемный обмен опытом во сне.**  
Синхронизация EXPERIENCE с другими экземплярами AXIOM через Memory Persistence в период DREAMING.

**Уровни сна (REM/non-REM аналог).**  
Разные виды переработки в разных фазах одного сна. Разная глубина, разные разрешённые операции.

**Сны на разных уровнях FractalChain.**  
Если у нас несколько уровней AshtiCore — спят ли они синхронно или каждый со своим ритмом? V1.0 предполагает один уровень.

**Принудительное обнуление EXPERIENCE через сон.**  
GC-аналог: сон с очисткой накопленного мусора в EXPERIENCE по порогу давления. Архитектурный вопрос (что считать мусором).

**`GatewayPriority::Emergency`.**  
В V1.0 ведёт себя как Critical. Отдельная семантика (немедленное прерывание DreamCycle без завершения текущего proposal) — deferred.

**`HeavyCrystallization` proposals.**  
`DreamProposalKind::HeavyCrystallization` объявлен в типах V1.0. В `cycle.rs` обрабатывается как Vetoed ("not implemented in V1.0"). Полная реализация требует спеки: что именно кристаллизуется тяжёлым путём, отличия от Promotion, взаимодействие с GUARDIAN.

---

## Технические долги из реализации V1.0

Найдены в коде, не являются архитектурными вопросами — конкретные TODO в существующих файлах.

**GENOME-проверка прав FrameWeaver.**  
`frame.rs:718`: `// TODO Phase 4: проверить права GENOME для ModuleId::FrameWeaver на EXPERIENCE и SUTRA`. В V1.0 проверка отсутствует — FrameWeaver пишет в EXPERIENCE и предлагает в SUTRA без верификации разрешений через GENOME. Требует: определить `ModuleId::FrameWeaver` в GENOME V2+, добавить проверку в `dream_propose()` и `on_tick()`.

**`RuleTrigger::HighConfidence` всегда false.**  
`frame.rs:604`: `// confidence не реализован в V1.1`. Правило `HighConfidence` в `FrameWeaverConfig.crystallization_rules` никогда не срабатывает. Требует: поле `confidence` в `FrameCandidate` (сейчас 0.0 заглушка) и интеграцию с Arbiter ML-моделью.

**Оптимизация `tick_dreaming`: тикать только DREAM(107) и EXPERIENCE(109).**  
`engine.rs:926`: `// V2.0: тикать только DREAM(107) и EXPERIENCE(109)`. В V1.0 `tick_dreaming` тикает всю физику AshtiCore как в WAKE. Оптимизация: в DREAMING имеет смысл тикать только домены, задействованные в DreamCycle. Снизит стоимость тика в DREAMING.
