# Axiom Roadmap

**Версия:** 3.1
**Дата:** 2026-03-19
**Текущая:** v0.4.0 Phase 2 - Causal Age

---

## 🎯 v0.4.0 - Causal Time System

### Phase 2: Causal Age (В РАБОТЕ)

**Задачи:**
- [ ] Проверить `last_event_id` в структурах
  - Token V5.2 ✅
  - Connection V5.0
  - DomainConfig V2.0
- [ ] Реализовать `compute_causal_age()`
  - Decay через causal age
  - Thermodynamics через event_id delta
  - Connection stress
  - Gravity
- [ ] Тесты

**Spec:** `docs/spec/time/Time_Model_V1_0.md`

---

### Phase 3: Causal Frontier

**Задачи:**
- [ ] `CausalFrontier` структура
  - Queue для Token/Connection/Domain
  - Visited BitSet
  - push/pop/contains
- [ ] PhysicsProcessor integration
- [ ] Storm detection/mitigation
- [ ] Тесты

**Spec:** `docs/spec/time/Causal Frontier System V1.md`

---

### Phase 4: Heartbeat

**Задачи:**
- [ ] `HeartbeatGenerator`
  - Генерация по счетчику событий
  - HeartbeatEvent структура
  - COM integration
- [ ] Frontier integration
- [ ] HeartbeatConfig в DomainConfig
- [ ] Тесты

**Spec:** `docs/spec/time/Heartbeat_V2_0.md`

---

### Phase 5: Cleanup & Polish

**Задачи:**
- [ ] Documentation updates
- [ ] Performance optimization
- [ ] Final testing
- [ ] Release preparation

---

## 📝 Принципы

- **STATUS.md** - только факты, завершенные релизы
- **ROADMAP.md** - только планы, удалять выполненное
- **DEFERRED.md** - технический долг и отложенные задачи
- **Минимализм** - краткость, структура, порядок

---

**Обновлено:** 2026-03-19
