# Axiom Roadmap

**Версия:** 11.0
**Дата:** 2026-03-28
**Спека:** [docs/spec/Roadmap_Ashti_Core_V2_1.md](docs/spec/Roadmap_Ashti_Core_V2_1.md)

---

## Сводка

| Этап | Название | Ключевой результат | Статус |
|------|----------|--------------------|--------|
| 1 | GENOME + GUARDIAN | Конституция, контроль доступа | ✅ 426 тестов |
| 2 | Storm Control | Защита от каскадов, state machine | ✅ 430 тестов |
| 3 | Configuration System | YAML для всего, снять hardcode | ✅ 469 тестов |
| 4 | REFLECTOR + SKILLSET | Статистика, кристаллизация скиллов | ✅ 496 тестов |
| 5 | GridHash | O(1) fast path, < 35 µs pipeline | ✅ 519 тестов |
| 6 | Адаптивные пороги | Самонастройка, DREAM(7) | ✅ 533 тестов |
| 7 | Causal Horizon | Долгий запуск, обмен скиллами | ✅ 568 тестов |
| 8 | External Integration | Gateway, Channel, adapter traits | ✅ 590 тестов |

Все этапы завершены. Технический долг и будущие планы: [DEFERRED.md](DEFERRED.md).

---

## 📝 Принципы

**Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)

- **STATUS.md** — только факты, завершённые релизы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Минимализм** — краткость, структура, порядок
