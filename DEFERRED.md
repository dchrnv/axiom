# Axiom - Отложенные задачи

**Версия:** 2.3
**Создан:** 2026-02-11
**Обновлен:** 2026-03-08

---

## Принцип ведения

Систематический учет всех заглушек и отложенных функций для будущих версий.
Каждая запись: где находится, что отложено, почему, когда планируется.

---

---

## 1. 🔥 КРИТИЧЕСКИЕ ПРОБЛЕМЫ - БЛОКИРУЮТ РАЗРАБОТКУ

### 1.1 Domain Configuration - TimeStrip Integration

**Где:** `runtime/src/domain.rs`, `docs/guides/UCL_V2.0_Guide.md`  
**Что отложено:** Конфигурация доменов использует timestamp вместо COM TimeStrip  
**Почему:** В гайде указаны timestamp, но в AXIOM используется COM TimeStrip система  
**Когда планируется:** v0.3.1 - Следующий релиз  

**Текущий статус:**
- **Проблема:** `created_at` и `last_update` используют timestamp (u64)
- **Реальность:** AXIOM использует COM TimeStrip (event_id) для временных меток
- **Последствия:** Несоответствие между гайдом и реальной реализацией

**Что требует исправления:**
- **Обновить гайд:** Заменить timestamp на COM event_id
- **Добавить TimeStrip функции:** Для создания и управления временными метками
- **Обновить factory методы:** Использовать COM TimeStrip вместо timestamp
- **Валидация:** Проверять корректность COM event_id

**Технические детали:**
```rust
// Текущий (неправильный) подход
domain.created_at = 1715292000; // timestamp
domain.last_update = 1715292000;

// Правильный подход с COM TimeStrip
domain.created_at = com.generate_event_id();
domain.last_update = com.get_current_event_id();
```

**Влияние на документацию:**
- Обновить все примеры в UCL_V2.0_Guide.md
- Обновить Configuration Instructions раздел
- Обновить Validation Rules для COM TimeStrip
- Добавить COM TimeStrip API reference

---

## 2. 🔧 **ЗАДАЧИ ПО УЛУЧШЕНИЮ - СРЕДНИЙ ПРИОРИТЕТ**

### 2.1 Factory Methods для всех доменов

**Где:** `runtime/src/domain.rs`  
**Что отложено:** Factory методы для MAP, PROBE, VOID, BRIDGE доменов  
**Почему:** Только 5 из 10 доменов имеют factory методы  
**Когда планируется:** v0.3.1 - Следующий релиз  

**Требуется реализовать:**
- `factory_map(domain_id, parent_id)` - Картография и навигация
- `factory_probe(domain_id, parent_id)` - Зонды и исследования
- `factory_void(domain_id, parent_id)` - Вакуум и пустота
- `factory_bridge(domain_id, parent_id)` - Мосты и связи

### 2.2 Events System Integration

**Где:** `runtime/src/`  
**Что отложено:** Система событий для COM интеграции  
**Почему:** UCL команды должны генерировать события  
**Когда планируется:** v0.3.2  

**Требуется:**
- Event структуры для DomainCreated, TokenInjected, ForceApplied
- Event bus для подписки и обработки
- Интеграция с PhysicsProcessor

---

## 3. 🟢 НИЗКИЙ ПРИОРИТЕТ - ДОЛГОСРОЧНЫЕ ЦЕЛИ

### 3.1 Python Adapter

**Где:** `runtime/src/python_adapter.rs`  
**Что отложено:** Python bindings для UCL V2.0  
**Почему:** Внешняя интеграция и CLI  
**Когда планируется:** v0.4.0  

### 3.2 REST API

**Где:** `runtime/src/rest_api.rs`  
**Что отложено:** HTTP endpoints для доменов  
**Почему:** Веб-интерфейс и удаленное управление  
**Когда планируется:** v0.4.0  

### 3.3 Performance Benchmarks

**Где:** `runtime/benches/`  
**Что отложено:** Бенчмарки UCL V2.0 производительности  
**Почему:** Измерение эффективности zero-allocation  
**Когда планируется:** v0.3.2  

---

## 4. 📚 **ДОКУМЕНТАЦИЯ И ГАЙДЫ**

### 4.1 API Documentation

**Где:** `docs/api/`  
**Что отложено:** Полная документация UCL V2.0 API  
**Почему:** Для внешних разработчиков  
**Когда планируется:** v0.3.1  

### 4.2 Examples and Tutorials

**Где:** `examples/`  
**Что отложено:** Примеры использования UCL V2.0  
**Почему:** Обучение и onboarding  
**Когда планируется:** v0.3.1  

---

## 📊 **СВОДКА ПО ПРИОРИТЕТАМ:**

### 🔥 КРИТИЧЕСКИЕ:
1. Domain Configuration - TimeStrip Integration (v0.3.1)

### 🔧 ВЫСОКИЙ:
2. Factory Methods для всех доменов (v0.3.1)
3. Events System Integration (v0.3.2)

### 🟢 СРЕДНИЙ/НИЗКИЙ:
4. Python Adapter (v0.4.0)
5. REST API (v0.4.0)
6. Performance Benchmarks (v0.3.2)
7. API Documentation (v0.3.1)
8. Examples and Tutorials (v0.3.1)

---

**Последнее обновление:** 2026-03-08  
**Создано в рамках:** Axiom Project  
**Статус:** UCL V2.0 Complete, roadmap для v0.3.1+

---
