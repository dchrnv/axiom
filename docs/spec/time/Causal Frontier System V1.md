## Causal Frontier System

### 1. Назначение

**Causal Frontier** — это структура управления вычислениями, определяющая **активную причинную границу системы**.

Frontier содержит только те элементы состояния, которые **могут породить новое событие**.

Система **никогда не выполняет глобальный проход по состоянию**.  
Все вычисления выполняются **только внутри frontier**.

Это гарантирует:

- масштабируемость
    
- локальность вычислений
    
- энергоэффективность
    
- возможность работы на слабом оборудовании.
    

---

# 2. Основные принципы

### 2.1 Локальность причинности

Любое событие влияет только на **ограниченную область состояния**.

Следовательно:

```
event → affected_entities → frontier
```

Frontier содержит только эти сущности.

---

### 2.2 Отсутствие глобальных обновлений

Запрещено:

```
scan_all_entities()
scan_all_shells()
scan_world()
```

Все проверки выполняются только для элементов frontier.

---

### 2.3 Детерминизм

Порядок обработки frontier должен быть **строго детерминированным**.

Разрешено:

```
stable queue
priority queue
ordered set
```

Запрещено:

```
random iteration
non-deterministic parallel traversal
```

---

# 3. Типы frontier

Frontier должен быть **типизированным**, чтобы избежать лишних проверок.

Минимальная структура:

```
CausalFrontier
    shell_frontier
    connection_frontier
    cluster_frontier
```

Каждый список содержит **идентификаторы сущностей состояния**.

---

# 4. Базовая структура

Минимальная реализация:

```
struct CausalFrontier {
    shells: Queue<ShellId>
    clusters: Queue<ClusterId>
    connections: Queue<ConnectionId>

    visited_shells: BitSet
    visited_clusters: BitSet
    visited_connections: BitSet
}
```

`visited` используется для **дедупликации элементов**.

---

# 5. Алгоритм обработки

Основной цикл симуляции:

```
while frontier not empty:

    entity = frontier.pop()

    evaluate_local_rules(entity)

    if transformation detected:
        event = generate_event()

        apply_event(event)

        affected = collect_neighbors(event)

        frontier.add(affected)
```

Ключевой принцип:

**обработка всегда локальна.**

---

# 6. Добавление элементов во frontier

Любое событие обязано добавить во frontier:

```
affected entity
direct neighbors
dependent structures
```

Пример:

```
ShellMerged(A, B)
```

добавляет:

```
A
B
neighbors(A)
neighbors(B)
cluster(A,B)
```

---

# 7. Пространственная оптимизация

Frontier должен использовать **spatial index** для поиска соседей.

Рекомендуемый механизм:

```
spatial hash grid
```

Схема работы:

```
frontier entity
      ↓
spatial index
      ↓
neighbors
```

Таким образом вычисляется только **локальная область взаимодействия**.

---

# 8. Idle state

Если frontier пуст:

```
frontier = ∅
```

система находится в состоянии **causal idle**.

В этом состоянии:

- симуляция не выполняется
    
- CPU не используется
    
- система ждёт внешнего события.
    

---

# 9. Causal Storms

## 9.1 Определение

**Causal Storm** — это ситуация, когда одно событие порождает лавину новых событий.

Пример:

```
cluster collapse
→ thousands of shell interactions
→ millions of checks
```

Без контроля это разрушает масштабируемость.

---

# 9.2 Storm detection

Система должна отслеживать:

```
events_per_cycle
frontier_growth_rate
```

Если:

```
frontier_size > STORM_THRESHOLD
```

включается режим **storm control**.

---

# 9.3 Storm mitigation

Система обязана применять минимум один механизм:

### Batch events

Схожие события объединяются.

Пример:

```
100 ShellMoved
→ BatchShellMoved
```

---

### Event aggregation

Мелкие трансформации объединяются.

Пример:

```
multiple micro-collisions
→ cluster relaxation event
```

---

### Causal budget

Ограничение вычислений на цикл.

```
max_events_per_cycle
```

Если лимит достигнут:

```
simulation paused
frontier persisted
resume next cycle
```

Это предотвращает зависание системы.

---

# 10. Frontier memory limits

Frontier обязан иметь **жёсткие ограничения памяти**.

```
max_frontier_size
```

При превышении:

```
apply storm mitigation
```

или

```
degrade simulation precision
```

---

# 11. Deterministic ordering

Если используется параллелизм, система должна гарантировать:

```
stable event ordering
```

Методы:

```
event_id ordering
domain partitioning
deterministic merge
```

---

# 12. Domain isolation

Каждый домен может иметь собственный frontier.

```
Domain
    state
    frontier
```

Междоменное взаимодействие происходит только через **события COM**.

---

# 13. Frontier lifecycle

Frontier проходит следующие стадии:

```
empty
active
storm
stabilized
idle
```

Это позволяет системе адаптировать стратегию вычислений.

---

# 14. Архитектурная роль

Frontier является **механизмом управления вычислениями**, а не частью модели мира.

Следовательно:

Frontier **не сохраняется в snapshot**.

Snapshot содержит только:

```
state
event log
```

Frontier восстанавливается из последних событий.

---

# 15. Минимальный интерфейс

Система должна предоставить:

```
push(entity)
pop()
contains(entity)
clear()
size()
```

И специализированные методы:

```
push_shell()
push_cluster()
push_connection()
```

---

# 16. Архитектурный результат

После введения Causal Frontier система приобретает свойства:

- O(active_entities) вычислительная сложность
    
- локальная причинность
    
- масштабируемость до миллионов сущностей
    
- естественное состояние сна
    
- устойчивость к причинным лавинам.
    

---

И последнее наблюдение, которое стоит держать в голове, даже если пока не фиксировать в спецификации.

В системах такого типа со временем почти всегда появляется структура, которую называют **causal horizon** — граница, после которой события уже не могут повлиять на текущую область симуляции.

Если когда-нибудь AXIOM вырастет до огромных миров, именно эта идея позволит **безопасно архивировать части истории**, не разрушая причинность.

Это один из редких архитектурных трюков, который превращает систему из «работает» в «работает на планетарном масштабе».