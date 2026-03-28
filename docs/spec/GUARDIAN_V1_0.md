# AXIOM MODULE SPECIFICATION: GUARDIAN V1.0

**Статус:** Актуальная спецификация (core)  
**Версия:** 1.0.0  
**Дата:** 2026-03-28  
**Назначение:** Над-доменный контроллер — исполнитель GENOME, управляющий CODEX  
**Crate:** `axiom-runtime` (часть оркестрации, не отдельный crate)  
**Модель времени:** COM `event_id` (генерирует события ингибирования)  
**Связанные спеки:** GENOME V1.0, Ashti_Core V2.0, Arbiter V1.0, DomainConfig V2.1, CODEX (Домен 3)  
**Наследие:** Guardian V1 из NeuroGraph + ADNA enforcement

---

## 1. Назначение

**GUARDIAN** — над-доменный модуль, исполняющий две конституции: неизменяемый **GENOME** и пластичный **CODEX (Домен 3)**.

GUARDIAN — **не домен**. У него нет поля, токенов, мембраны. Это процесс уровня Ashti_Core, наравне с Arbiter и связующими шинами.

GUARDIAN отвечает на вопрос: **"Допустимо ли это?"**  
Arbiter отвечает на вопрос: **"Куда и как направить?"**

Роли не пересекаются. Arbiter маршрутизирует. GUARDIAN фильтрует.

---

## 2. Два источника правил

GUARDIAN читает правила из двух источников с разным приоритетом:

### 2.1 GENOME (абсолютный приоритет)

Неизменяемый. Загружен при старте. Содержит архитектурные инварианты, права доступа, протоколы.

Нарушение GENOME → немедленное вето. Без обсуждений, без обратной связи, без исключений.

Примеры GENOME-правил:
- Только SUTRA(0) может создавать токены (sutra_write_exclusive).
- event_id строго монотонно возрастает (event_id_monotonic).
- Маршрут EXPERIENCE(9) → LOGIC(6) не существует в протоколах → запрещён.

### 2.2 CODEX (Домен 3, пластичный)

Живой домен с полем, токенами, физикой. Содержит поведенческие правила, которые система выучила или получила.

Нарушение CODEX → вето, но правило может быть обновлено. GUARDIAN — единственный модуль с правом Write на CODEX.

Примеры CODEX-правил:
- "Токен с valence < -100 не должен проходить в MAYA(10) без обработки в ASHTI(5) Ethics."
- "Рефлекс с weight < 0.3 не считается достоверным для домена LOGIC(6)."

CODEX-правила — это токены и связи внутри домена 3. Они подчиняются физике поля (гравитация, затухание, температура). Старые правила затухают. Подтверждённые — усиливаются. Это живой закон.

---

## 3. Функции GUARDIAN

### 3.1 Проверка рефлексов (Reflex Validation)

Когда Arbiter классифицирует ответ EXPERIENCE(9) как рефлекс и бит `GUARDIAN_CHECK_REQUIRED` установлен в DomainConfig:

```
EXPERIENCE(9) → Arbiter → [GUARDIAN проверка] → MAYA(10)
```

GUARDIAN выполняет **облегчённую проверку** (не полную валидацию как для обычных узоров):

```rust
pub fn validate_reflex(
    &self,
    reflex: &ReflexPayload,
    target_domain: u16,
) -> ReflexDecision {
    // 1. GENOME: проверить протокол (допустим ли маршрут 9 → 10?)
    if !self.genome.check_protocol(ModuleId::Experience, ModuleId::Maya, DataType::Reflex) {
        return ReflexDecision::Veto(VetoReason::GenomeProtocolViolation);
    }

    // 2. GENOME: проверить права (имеет ли Arbiter право Execute на MAYA?)
    if !self.genome.check_access(ModuleId::Arbiter, ResourceId::Maya, Permission::Execute) {
        return ReflexDecision::Veto(VetoReason::GenomeAccessViolation);
    }

    // 3. CODEX: проверить поведенческие правила
    if let Some(violation) = self.check_codex_rules(reflex, target_domain) {
        return ReflexDecision::Veto(VetoReason::CodexViolation(violation));
    }

    ReflexDecision::Approve
}
```

Решение: **Approve** (рефлекс отправляется в MAYA) или **Veto** (рефлекс подавляется, Arbiter понижает его до ассоциации или тишины).

### 3.2 Сканирование узоров (Pattern Scanning)

GUARDIAN периодически сканирует состояния доменов через `peek_state()`:

```rust
pub trait Scannable {
    /// Даёт GUARDIAN read-only доступ к текущему состоянию домена.
    /// Не модифицирует состояние. Не генерирует событий.
    fn peek_state(&self) -> &DomainState;
}
```

GUARDIAN сканирует узоры (паттерны, формирующиеся в доменах) и ингибирует те, которые нарушают GENOME или CODEX:

```rust
pub fn scan_domain(&self, domain: &impl Scannable, domain_id: u16) -> Vec<InhibitAction> {
    let state = domain.peek_state();
    let mut actions = Vec::new();

    for (idx, token) in state.tokens.iter().enumerate() {
        // Проверка GENOME-инвариантов
        if token.mass == 0 {
            actions.push(InhibitAction::Token(idx as u32, InhibitReason::GenomeInvariant("mass > 0")));
        }

        // Проверка CODEX-правил
        if let Some(reason) = self.check_codex_for_token(token, domain_id) {
            actions.push(InhibitAction::Token(idx as u32, InhibitReason::CodexRule(reason)));
        }
    }

    actions
}
```

Ингибирование — это **не удаление**. GUARDIAN не редактирует данные. Он запрещает исполнение: устанавливает флаг `INHIBITED` на токене или связи. Домен видит ингибированные сущности, но не обрабатывает их.

### 3.3 Управление CODEX (CODEX Write)

GUARDIAN — единственный модуль с правом Write на CODEX (Домен 3).

Когда GUARDIAN обнаруживает:
- Рефлекс систематически расходится с результатами 1-8 → GUARDIAN ослабляет соответствующее правило в CODEX.
- Новый паттерн подтверждён многократно → GUARDIAN усиливает правило или создаёт новое.
- Правило в CODEX противоречит GENOME → GUARDIAN ингибирует правило.

```rust
pub fn update_codex(
    &self,
    codex_domain: &mut DomainState,
    action: CodexAction,
) -> Result<(), GuardianError> {
    match action {
        CodexAction::StrengthenRule(rule_id, delta) => {
            // Усилить токен-правило в CODEX
            let token = &mut codex_domain.tokens[rule_id as usize];
            token.mass = token.mass.saturating_add(delta);
        }
        CodexAction::WeakenRule(rule_id, delta) => {
            // Ослабить правило (но не ниже min_intensity)
            let token = &mut codex_domain.tokens[rule_id as usize];
            token.mass = token.mass.saturating_sub(delta).max(1);
        }
        CodexAction::InhibitRule(rule_id) => {
            // Заблокировать правило (установить INHIBITED флаг)
            let token = &mut codex_domain.tokens[rule_id as usize];
            token.state = TokenState::Locked;
        }
        CodexAction::CreateRule(new_token) => {
            // Создать новое правило в CODEX
            codex_domain.add_token(new_token)?;
        }
    }
    Ok(())
}
```

Все действия GUARDIAN на CODEX генерируют COM-события: `CodexRuleStrengthened`, `CodexRuleWeakened`, `CodexRuleInhibited`, `CodexRuleCreated`.

### 3.4 Проверка доступа (Access Enforcement)

GUARDIAN проверяет каждое межмодульное взаимодействие на соответствие GENOME:

```rust
pub fn enforce_access(
    &self,
    module: ModuleId,
    resource: ResourceId,
    operation: Permission,
) -> bool {
    // O(1) lookup через предвычисленную матрицу из GENOME
    self.genome_index.access_matrix[module as usize][resource as usize] >= operation
}
```

Стоимость: один доступ к памяти (~1 ns). Не bottleneck.

### 3.5 Проверка протоколов (Protocol Enforcement)

GUARDIAN проверяет что маршрут данных разрешён в GENOME:

```rust
pub fn enforce_protocol(
    &self,
    source: ModuleId,
    target: ModuleId,
) -> bool {
    // O(1) lookup через предвычисленную матрицу из GENOME
    self.genome_index.protocol_matrix[source as usize][target as usize]
}
```

---

## 4. Структура

```rust
pub struct Guardian {
    /// Неизменяемая ссылка на GENOME (shared with all modules)
    genome: Arc<Genome>,

    /// Предвычисленный индекс для O(1) проверок
    genome_index: GenomeIndex,

    /// Буфер для результатов сканирования (предвыделён)
    scan_buffer: Vec<InhibitAction>,

    /// Статистика (для мониторинга)
    stats: GuardianStats,
}

pub struct GuardianStats {
    pub reflexes_approved: u64,
    pub reflexes_vetoed: u64,
    pub patterns_inhibited: u64,
    pub codex_rules_updated: u64,
}
```

---

## 5. Интеграция

### 5.1 С Arbiter

Arbiter запрашивает GUARDIAN для проверки рефлексов. Взаимодействие синхронное (Arbiter ждёт ответа):

```
Arbiter: "Рефлекс X для домена EXECUTION(1) — допустимо?"
GUARDIAN: проверяет GENOME (протокол, доступ) + CODEX (поведенческие правила)
GUARDIAN → Arbiter: Approve / Veto(причина)
```

### 5.2 С EXPERIENCE (Домен 9)

GUARDIAN может ингибировать следы в EXPERIENCE(9), если обнаруживает что рефлекс систематически ошибается (защита от деградации рефлексов, Ashti_Core V2.0, раздел 7.2):

```
GUARDIAN обнаруживает: рефлекс R расходится с результатом 1-8 уже 5 раз подряд
→ GUARDIAN принудительно снижает weight следа R в EXPERIENCE(9)
→ Или помечает след для повторной валидации
```

### 5.3 С CODEX (Домен 3)

GUARDIAN — единственный писатель в CODEX. Все изменения правил проходят через GUARDIAN:

- Обратная связь из MAYA → Arbiter → GUARDIAN → CODEX (усиление/ослабление правил).
- Обнаружение противоречий GENOME vs CODEX → GUARDIAN ингибирует правило в CODEX.
- Кристаллизация скиллов в EXPERIENCE(9) → GUARDIAN может создать новое правило в CODEX.

### 5.4 С Heartbeat

GUARDIAN выполняет периодическое сканирование доменов. Это фоновый процесс, привязанный к Heartbeat:

```
HeartbeatEvent → GUARDIAN сканирует домен N → ингибирует нарушающие узоры
```

За `ceil(ashti_domain_count / scan_batch_size)` пульсов все домены будут проверены. Нагрузка распределена.

### 5.5 С COM

GUARDIAN генерирует COM-события для всех своих действий:

```rust
pub enum GuardianEventType {
    ReflexApproved,        // Рефлекс одобрен
    ReflexVetoed,          // Рефлекс заблокирован
    PatternInhibited,      // Узор ингибирован
    CodexRuleUpdated,      // Правило в CODEX изменено
    AccessViolationDetected, // Обнаружена попытка неавторизованного доступа
    ProtocolViolationDetected, // Обнаружен неразрешённый маршрут
}
```

Все действия GUARDIAN — в причинном порядке, детерминированны и воспроизводимы.

---

## 6. Производительность

### 6.1 Проверки GENOME (access, protocol)

O(1) через предвычисленную матрицу. ~1-2 ns. Не bottleneck.

### 6.2 Проверка рефлексов

Облегчённая: GENOME checks (O(1)) + CODEX checks (зависит от количества активных правил в CODEX). При 100 правилах в CODEX: ожидаемое время ~100-500 ns. Укладывается в бюджет Arbiter (~4 µs).

### 6.3 Сканирование доменов

Фоновый процесс через Heartbeat. Не горячий путь. Сканирование одного домена с 100 токенами: ~10 µs (линейный проход + проверки).

---

## 7. Инварианты

1. **GENOME абсолютен.** GUARDIAN не может нарушить GENOME. Если проверка GENOME возвращает вето — вето окончательно.
2. **CODEX пластичен.** GUARDIAN может изменять CODEX, но только в рамках, определённых GENOME.
3. **Единственный писатель.** GUARDIAN — единственный модуль с правом Write на CODEX. Никто другой не может менять правила.
4. **Не редактирует данные.** GUARDIAN ингибирует, но не удаляет. Он устанавливает флаги, а не стирает токены.
5. **COM-совместимость.** Все действия GUARDIAN генерируют COM-события. Детерминизм и воспроизводимость гарантированы.
6. **Stateless (почти).** GUARDIAN не хранит состояние между вызовами, кроме GuardianStats (статистика). Все решения принимаются на основе текущего состояния GENOME + CODEX.

---

## 8. Связь с Trusted Reflex (из NeuroGraph)

В NeuroGraph IntuitionEngine v3.0 вводил понятие "Trusted Reflex" — рефлекс с высоким confidence, проходящий облегчённую проверку Guardian.

В AXIOM это реализуется через бит `GUARDIAN_CHECK_REQUIRED` в DomainConfig V2.1 (arbiter_flags, бит 3):

- Если бит установлен — рефлекс проходит через GUARDIAN (полная проверка).
- Если бит не установлен — рефлекс считается "доверенным" и отправляется в MAYA без проверки GUARDIAN. Arbiter проверяет только reflex_threshold и cooldown.

Рекомендация: для большинства доменов бит `GUARDIAN_CHECK_REQUIRED` должен быть установлен. "Доверенные" рефлексы — только для доменов с очень высоким reflex_threshold (> 200), где confidence уже экстремально высокий.

---

## 9. История изменений

- **V1.0**: Первая версия. Два источника правил (GENOME + CODEX). Пять функций: reflex validation, pattern scanning, CODEX management, access enforcement, protocol enforcement. Интеграция с Arbiter, EXPERIENCE, Heartbeat, COM. Связь с Trusted Reflex из NeuroGraph.
