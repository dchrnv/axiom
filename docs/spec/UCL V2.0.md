# AXIOM PROTOCOL: UCL (Unified Command Language) V1.1

**Статус:** Core Communication Protocol

**Версия:** 2.0 (Zero-Allocation FFI Frame)

**Формат:** 64 байта, `repr(C, align(64))`

**Связанные спеки:** DomainConfig V1.1, Ashti_Core

---

## 1. Назначение и Философия

**UCL** — это единственный способ изменить состояние пространства AXIOM.

Команды не передаются в виде JSON. Внешние адаптеры (Python, REST, CLI) парсят пользовательский ввод (текст/JSON) на своей стороне, формируют строгий **64-байтный бинарный фрейм** и передают его в Rust-ядро через разделяемую память (FFI) или сокет.

**Принципы:**

1. **Zero-Allocation:** Команды не содержат `String`, `Vec` или `Option`. Размер строго 64 байта.
    
2. **Физическая семантика:** Мы не "апдейтим базу". Мы "прикладываем силу" или "рождаем домен".
    
3. **Изоляция:** Ядро не знает про пользователей или HTTP-сессии. Оно знает только про векторы, массы и домены.
    

---

## 2. Структура Фрейма (Command Frame) - 64 байта

Структура выровнена по границе 64 байт (идеально для одной кэш-линии процессора).

Rust

```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct UclCommand {
    // --- ЗАГОЛОВОК [16 байт] ---
    pub command_id: u64,        // 8b | Уникальный ID транзакции (COM)
    pub opcode: u16,            // 2b | Тип команды (CommandType)
    pub target_id: u32,         // 4b | Цель (Domain ID или Token ID)
    pub priority: u8,           // 1b | 0 (Low) - 255 (Critical)
    pub flags: u8,              // 1b | Битовая маска (Sync, Force, Bypass_Membrane)

    // --- ПОЛЕЗНАЯ НАГРУЗКА (PAYLOAD) [48 байт] ---
    // Используем raw union или unsafe трансляцию, 
    // чтобы разные Payload занимали один и тот же участок памяти.
    pub payload: [u8; 48],      
}
```

---

## 3. Словарь Команд (OpCodes)

Совмещаем твои CRUD-операции с нашей физикой.

Rust

```
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OpCode {
    // --- Генезис и Пространство (1000+) ---
    SpawnDomain = 1000,      // Заменяет CreateDomain
    CollapseDomain = 1001,   // Заменяет DeleteDomain
    LockMembrane = 1002,     // Заменяет UpdateDomain (изменение фильтров)
    
    // --- Токены и Кинематика (2000+) ---
    InjectToken = 2000,      // Вброс нового смысла
    ApplyForce = 2001,       // Векторный толчок токена (Move)
    AnnihilateToken = 2002,  // Уничтожение токена
    
    // --- Хронодинамика и Система (3000+) ---
    TickForward = 3000,      // Шаг симуляции
    ChangeTemperature = 3001,// Изменение термодинамики
    CoreShutdown = 9000,     // Остановка реактора
}
```

---

## 4. Структуры Нагрузки (Payloads)

Эти структуры в Rust кастуются прямо из `payload: [u8; 48]`. Никакого `serde`.

### 4.1 SpawnDomainPayload (Генезис)

Rust

```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpawnDomainPayload {
    pub parent_domain_id: u16,  // 2b
    pub factory_preset: u8,     // 1b | 0=Void, 1=Sutra, 6=Logic, 7=Dream, 10=Maya
    pub structural_role: u8,    // 1b | Для валидации
    // Остальные 44 байта заполнены нулями (резерв)
}
```

### 4.2 ApplyForcePayload (Кинематика)

Rust

```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ApplyForcePayload {
    pub force_vector: [f32; 3], // 12b | Направление X, Y, Z
    pub magnitude: f32,         // 4b  | Сила импульса
    pub duration_ticks: u32,    // 4b  | Как долго действует сила
}
```

---

## 5. Command Result (Ответ ядра)

Ядро отвечает не длинными JSON-строками с ошибками, а жестким 32-байтным статусом. Если внешнему адаптеру нужны тексты ошибок, он переводит коды в строки на своей стороне (в Python/REST).

Rust

```
#[repr(C, align(32))]
#[derive(Debug, Clone, Copy)]
pub struct UclResult {
    pub command_id: u64,        // 8b | Ссылка на исходную команду
    pub status: u8,             // 1b | 0=Success, 1=PhysicsViolation, 2=TargetNotFound
    pub error_code: u16,        // 2b | Детальный код аномалии
    pub consumed_energy: f32,   // 4b | Затраченная энергия Домена на операцию
    pub events_generated: u16,  // 2b | Кол-во порожденных событий в шине COM
    pub reserved: [u8; 15],     // 15b| Добивка до 32 байт
}
```

---

## 6. Как это работает в архитектуре (Интеграция)

Твоя идея изолировать адаптеры работает здесь на 100%. Мы просто проводим черту между "миром людей" и "миром физики".

### Внешний слой (REST / Python / Maya Layer)

Здесь работают пользователи, JSON, токены авторизации и базы данных.

Python

```
# REST Endpoint на Python (FastAPI)
@app.post("/api/domains")
async def create_domain(request: CreateDomainRequest, user=Depends(get_user)):
    # 1. Валидация прав доступа (user_id) происходит ЗДЕСЬ, вне ядра.
    if not user.can_create_sutra():
        return HTTP_403
    
    # 2. Формируем 64-байтный бинарный фрейм
    frame = UclBuilder.spawn_domain(
        target_id=request.domain_id,
        preset=Factories.SUTRA
    )
    
    # 3. Стреляем в Rust-ядро через разделяемую память (Zero-copy)
    result_bytes = axiom_core.execute_frame(frame)
    
    # 4. Расшифровываем ответ ядра и отдаем пользователю красивый JSON
    return parse_ucl_result(result_bytes)
```

### Внутренний слой (Rust Core / CommandProcessor)

Ядро не тратит процессорное время на проверку токенов доступа. Оно просто исполняет физику.

Rust

```
pub trait PhysicsProcessor {
    /// Главная точка входа. Принимает ровно 64 байта.
    fn execute(&mut self, command: &UclCommand) -> UclResult {
        match command.opcode {
            1000 => self.spawn_domain(command),
            2001 => self.apply_force(command),
            _ => UclResult::error(ERR_UNKNOWN_OPCODE),
        }
    }
}
```

---

### Резюме:

Мы взяли твою идею (CQRS, Command Processor, независимость от транспорта) и очистили её от веб-зависимостей (`String`, `Serialize`).

Теперь:

1. **Быстро:** Команды весят 64 байта. Никаких аллокаций памяти.
    
2. **Безопасно:** Пользователи и сессии проверяются на уровне Python/API, ядро этим не загружено.
    
3. **Физично:** Команды отражают законы пространства Axiom.
    

Что скажешь? Готов перевести этот протокол на уровень кода и отдать его SWE-агенту для интеграции в `Ashti_Core`?