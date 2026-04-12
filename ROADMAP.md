# Axiom Roadmap

**Версия:** 28.0
**Дата:** 2026-04-12

---

## Текущее состояние

Axiom Sentinel V1.0 завершён. CLI Extended V1.0 (Фазы 1 и 2) завершены.
Технический долг D-01/D-02/D-03/D-05 закрыт. Все тесты зелёные.

```
axiom-core → axiom-arbiter → axiom-domain → axiom-runtime
                                                    ↑
axiom-config → axiom-genome → axiom-frontier    axiom-persist
axiom-space → axiom-shell → axiom-heartbeat         ↑
axiom-ucl → axiom-upo                          axiom-agent
                                                (axiom-cli)
```

---

## Следующие задачи

### CLI Extended V1.0 — Фаза 3

**Что:** Детализация конкретных объектов и статистика.

| Команда | Описание |
|---------|----------|
| `:trace <index>` | Детали одного experience trace |
| `:connections [domain_id]` | Связи в домене |
| `:dream` | Состояние DREAM(7): последний анализ |
| `:multipass` | Статистика multi-pass событий |
| `:reflector` | Per-domain accuracy REFLECTOR |
| `:impulses` | Очередь pending_impulses |
| `:help <command>` | Детали конкретной команды |

**Спек:** [CLI_Extended_Commands_V1.md](CLI_Extended_Commands_V1.md) — Фаза 3

---

## Принципы

- **STATUS.md** — только факты, завершённые этапы
- **ROADMAP.md** — только планы, удалять выполненное
- **DEFERRED.md** — технический долг и отложенные задачи
- **Правила разработки:** [DEVELOPMENT_GUIDE.md](DEVELOPMENT_GUIDE.md)
