# Axiom Roadmap

**Версия:** 9.4
**Дата:** 2026-03-27

---

## 🔮 Долгосрочные цели

### axiom-upo тесты
UPO v2.2 мигрирован без тестов. Покрыть: `DynamicTrace`, `UPOEngine::record_*`, `generate_patch`. Низкий приоритет.

### Configuration System
YAML-загрузка пространственных параметров и semantic_contributions. Требует согласования с DomainConfig 128-byte constraint.

### Адаптеры
Python bindings, REST API, gRPC — нужны для внешней интеграции.

### Производительность
SIMD (AVX-512), incremental spatial hash rebuild — после стабилизации архитектуры.

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок

---

**Обновлено:** 2026-03-27
