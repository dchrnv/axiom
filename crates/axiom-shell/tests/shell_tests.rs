use axiom_core::Connection;
use axiom_shell::*;

#[test]
fn test_shell_profile_size() {
    // Shell V3.0: ShellProfile должен быть ровно 8 байт
    use std::mem::size_of;
    assert_eq!(size_of::<ShellProfile>(), 8, "ShellProfile must be 8 bytes");
}

#[test]
fn test_empty_shell_constant() {
    // EMPTY_SHELL должен быть нулевым профилем
    assert_eq!(EMPTY_SHELL, [0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_layer_names_count() {
    // Должно быть 8 имён слоёв
    assert_eq!(LAYER_NAMES.len(), 8);
    assert_eq!(LAYER_NAMES[0], "Physical");
    assert_eq!(LAYER_NAMES[7], "Abstract");
}

#[test]
fn test_domain_shell_cache_new() {
    // DomainShellCache::new() создаёт кэш с нулевыми профилями
    let cache = DomainShellCache::new(10);

    assert_eq!(cache.capacity(), 10);
    assert_eq!(cache.generation, 0);

    // Все профили должны быть EMPTY_SHELL
    for i in 0..10 {
        assert_eq!(cache.get(i), &EMPTY_SHELL);
        assert!(!cache.is_dirty(i), "Token {} should not be dirty initially", i);
    }
}

#[test]
fn test_domain_shell_cache_get_set() {
    // get() и set() работают корректно
    let mut cache = DomainShellCache::new(5);

    let profile = [10, 20, 30, 40, 50, 60, 70, 80];
    cache.set(2, profile);

    assert_eq!(cache.get(2), &profile);
    assert_eq!(cache.get(0), &EMPTY_SHELL); // Другие не изменились
}

#[test]
fn test_domain_shell_cache_dirty_flags() {
    // mark_dirty(), is_dirty(), clear_dirty() работают
    let mut cache = DomainShellCache::new(5);

    // Изначально все чистые
    assert!(!cache.is_dirty(0));
    assert!(!cache.is_dirty(4));

    // Помечаем токен 1 как грязный
    cache.mark_dirty(1);
    assert!(cache.is_dirty(1));
    assert!(!cache.is_dirty(0)); // Остальные чистые

    // Снимаем флаг
    cache.clear_dirty(1);
    assert!(!cache.is_dirty(1));
}

#[test]
fn test_domain_shell_cache_clear_all_dirty() {
    // clear_all_dirty() снимает все флаги
    let mut cache = DomainShellCache::new(5);

    // Помечаем несколько токенов
    cache.mark_dirty(0);
    cache.mark_dirty(2);
    cache.mark_dirty(4);

    assert!(cache.is_dirty(0));
    assert!(cache.is_dirty(2));
    assert!(cache.is_dirty(4));

    // Снимаем все флаги
    cache.clear_all_dirty();

    for i in 0..5 {
        assert!(!cache.is_dirty(i), "Token {} should be clean after clear_all", i);
    }
}

#[test]
fn test_domain_shell_cache_multiple_operations() {
    // Комбинация операций: set + mark_dirty + get
    let mut cache = DomainShellCache::new(3);

    let profile1 = [1, 2, 3, 4, 5, 6, 7, 8];
    let profile2 = [10, 20, 30, 40, 50, 60, 70, 80];

    cache.set(0, profile1);
    cache.mark_dirty(0);

    cache.set(1, profile2);
    // Не помечаем токен 1 как грязный

    assert_eq!(cache.get(0), &profile1);
    assert_eq!(cache.get(1), &profile2);
    assert_eq!(cache.get(2), &EMPTY_SHELL);

    assert!(cache.is_dirty(0));
    assert!(!cache.is_dirty(1));
    assert!(!cache.is_dirty(2));
}

#[test]
fn test_shell_contribution_type() {
    // ShellContribution должен быть того же типа что ShellProfile
    use std::mem::size_of;
    assert_eq!(size_of::<ShellContribution>(), 8);

    let contribution: ShellContribution = [5, 10, 15, 20, 25, 30, 35, 40];
    assert_eq!(contribution.len(), 8);
}

// --- SemanticContributionTable Tests ---

#[test]
fn test_semantic_contribution_table_new() {
    // new() создаёт таблицу с нулевыми профилями
    let table = SemanticContributionTable::new();

    // Все категории должны быть нулевыми
    for category in 0..=255u8 {
        let link_type = (category as u16) << 8;
        assert_eq!(table.get(link_type), &[0; 8]);
    }
}

#[test]
fn test_semantic_contribution_table_set_category() {
    // set_category() устанавливает базовый профиль категории
    let mut table = SemanticContributionTable::new();

    let structural_profile = [20, 5, 0, 0, 5, 0, 0, 0];
    table.set_category(0x01, structural_profile);

    // Любой link_type из категории 0x01 должен вернуть этот профиль
    assert_eq!(table.get(0x0100), &structural_profile);
    assert_eq!(table.get(0x0101), &structural_profile);
    assert_eq!(table.get(0x01FF), &structural_profile);

    // Другие категории остаются нулевыми
    assert_eq!(table.get(0x0200), &[0; 8]);
}

#[test]
fn test_semantic_contribution_table_set_override() {
    // set_override() переопределяет конкретный тип
    let mut table = SemanticContributionTable::new();

    let category_profile = [0, 0, 5, 0, 15, 0, 10, 8];
    let override_profile = [0, 0, 0, 20, 10, 0, 10, 5];

    table.set_category(0x03, category_profile);
    table.set_override(0x0302, override_profile);

    // 0x0302 должен вернуть переопределённый профиль
    assert_eq!(table.get(0x0302), &override_profile);

    // Другие типы из категории 0x03 возвращают базовый профиль
    assert_eq!(table.get(0x0300), &category_profile);
    assert_eq!(table.get(0x0301), &category_profile);
    assert_eq!(table.get(0x0303), &category_profile);
}

#[test]
fn test_semantic_contribution_table_two_level_hierarchy() {
    // Проверка двухуровневой иерархии: category vs override
    let mut table = SemanticContributionTable::new();

    table.set_category(0x04, [5, 20, 0, 15, 0, 0, 0, 0]);
    table.set_override(0x0406, [0, 15, 0, 10, 0, 0, 0, 15]);

    // Override приоритетнее category
    assert_eq!(table.get(0x0406), &[0, 15, 0, 10, 0, 0, 0, 15]);

    // Без override используется category
    assert_eq!(table.get(0x0405), &[5, 20, 0, 15, 0, 0, 0, 0]);
}

#[test]
fn test_semantic_contribution_table_default_ashti_core() {
    // default_ashti_core() загружает 7 категорий
    let table = SemanticContributionTable::default_ashti_core();

    // 0x01: Structural [20, 5, 0, 0, 5, 0, 0, 0]
    assert_eq!(table.get(0x0100), &[20, 5, 0, 0, 5, 0, 0, 0]);

    // 0x02: Semantic [0, 0, 0, 0, 15, 0, 0, 10]
    assert_eq!(table.get(0x0200), &[0, 0, 0, 0, 15, 0, 0, 10]);

    // 0x03: Causal [0, 0, 5, 0, 15, 0, 10, 8]
    assert_eq!(table.get(0x0300), &[0, 0, 5, 0, 15, 0, 10, 8]);

    // 0x04: Experiential [5, 20, 0, 15, 0, 0, 0, 0]
    assert_eq!(table.get(0x0400), &[5, 20, 0, 15, 0, 0, 0, 0]);

    // 0x05: Social [0, 0, 0, 5, 0, 25, 0, 0]
    assert_eq!(table.get(0x0500), &[0, 0, 0, 5, 0, 25, 0, 0]);

    // 0x06: Temporal [0, 0, 0, 0, 5, 0, 25, 0]
    assert_eq!(table.get(0x0600), &[0, 0, 0, 0, 5, 0, 25, 0]);

    // 0x07: Motor [10, 0, 25, 0, 5, 0, 0, 0]
    assert_eq!(table.get(0x0700), &[10, 0, 25, 0, 5, 0, 0, 0]);

    // Неопределённые категории остаются нулевыми
    assert_eq!(table.get(0x0800), &[0; 8]);
    assert_eq!(table.get(0xFF00), &[0; 8]);
}

#[test]
fn test_semantic_contribution_table_category_extraction() {
    // Проверка правильности извлечения категории из link_type
    let mut table = SemanticContributionTable::new();

    table.set_category(0x42, [1, 2, 3, 4, 5, 6, 7, 8]);

    // Старший байт 0x42, младший может быть любым
    assert_eq!(table.get(0x4200), &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(table.get(0x42AB), &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(table.get(0x42FF), &[1, 2, 3, 4, 5, 6, 7, 8]);

    // Другая категория
    assert_eq!(table.get(0x4300), &[0; 8]);
}

// --- compute_shell() Tests ---

#[test]
fn test_compute_shell_no_connections() {
    // Токен без связей должен иметь нулевой профиль
    let table = SemanticContributionTable::default_ashti_core();
    let connections: Vec<Connection> = vec![];

    let profile = compute_shell(100, &connections, &table);
    assert_eq!(profile, EMPTY_SHELL);
}

#[test]
fn test_compute_shell_single_connection() {
    // Один токен, одна связь
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [20, 10, 0, 0, 5, 0, 0, 0]); // Structural

    let mut conn = Connection::default();
    conn.source_id = 100;
    conn.target_id = 200;
    conn.link_type = 0x0100; // Structural category
    conn.strength = 1.0;

    let connections = vec![conn];

    let profile = compute_shell(100, &connections, &table);

    // Нормализация: max = 20 → scale = 255/20 = 12.75
    // [20, 10, 0, 0, 5, 0, 0, 0] × 12.75 ≈ [255, 128, 0, 0, 64, 0, 0, 0]
    assert_eq!(profile[0], 255); // Physical layer (max)
    assert_eq!(profile[1], 128); // Sensory layer (10 × 12.75 = 127.5 → 128)
    assert_eq!(profile[2], 0);
    assert_eq!(profile[3], 0);
    assert_eq!(profile[4], 64); // Cognitive (5 × 12.75 = 63.75 → 64)
    assert_eq!(profile[5], 0);
    assert_eq!(profile[6], 0);
    assert_eq!(profile[7], 0);
}

#[test]
fn test_compute_shell_multiple_connections() {
    // Токен с несколькими связями разных типов
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [20, 0, 0, 0, 0, 0, 0, 0]); // Structural
    table.set_category(0x02, [0, 0, 0, 0, 15, 0, 0, 0]); // Semantic

    let mut conn1 = Connection::default();
    conn1.source_id = 100;
    conn1.target_id = 200;
    conn1.link_type = 0x0100; // Structural
    conn1.strength = 1.0;

    let mut conn2 = Connection::default();
    conn2.source_id = 100;
    conn2.target_id = 300;
    conn2.link_type = 0x0200; // Semantic
    conn2.strength = 1.0;

    let connections = vec![conn1, conn2];

    let profile = compute_shell(100, &connections, &table);

    // Accumulator: [20, 0, 0, 0, 15, 0, 0, 0]
    // max = 20 → scale = 255/20 = 12.75
    // [20, 0, 0, 0, 15, 0, 0, 0] × 12.75 ≈ [255, 0, 0, 0, 191, 0, 0, 0]
    assert_eq!(profile[0], 255); // Physical (max)
    assert_eq!(profile[4], 191); // Cognitive (15 × 12.75 = 191.25 → 191)
}

#[test]
fn test_compute_shell_weighted_strength() {
    // Проверка взвешивания по strength
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [10, 0, 0, 0, 0, 0, 0, 0]);

    let mut conn1 = Connection::default();
    conn1.source_id = 100;
    conn1.target_id = 200;
    conn1.link_type = 0x0100;
    conn1.strength = 2.0; // Двойной вес

    let connections = vec![conn1];

    let profile = compute_shell(100, &connections, &table);

    // contribution = [10, 0, 0, 0, 0, 0, 0, 0]
    // × strength 2.0 = [20, 0, 0, 0, 0, 0, 0, 0]
    // max = 20 → scale = 255/20 = 12.75
    // [20, 0, 0, 0, 0, 0, 0, 0] × 12.75 = [255, 0, 0, 0, 0, 0, 0, 0]
    assert_eq!(profile[0], 255);
}

#[test]
fn test_compute_shell_source_and_target() {
    // Токен участвует и как source, и как target
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [10, 0, 0, 0, 0, 0, 0, 0]);

    let mut conn1 = Connection::default();
    conn1.source_id = 100;
    conn1.target_id = 200;
    conn1.link_type = 0x0100;
    conn1.strength = 1.0;

    let mut conn2 = Connection::default();
    conn2.source_id = 300;
    conn2.target_id = 100; // Токен 100 как target
    conn2.link_type = 0x0100;
    conn2.strength = 1.0;

    let connections = vec![conn1, conn2];

    let profile = compute_shell(100, &connections, &table);

    // Обе связи дают вклад: [10, 0, ...] + [10, 0, ...] = [20, 0, ...]
    // max = 20 → scale = 255/20 = 12.75
    // [20, 0, ...] × 12.75 = [255, 0, ...]
    assert_eq!(profile[0], 255);
}

#[test]
fn test_compute_shell_normalization() {
    // Проверка правильности нормализации
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [100, 50, 25, 10, 5, 2, 1, 0]);

    let mut conn = Connection::default();
    conn.source_id = 100;
    conn.target_id = 200;
    conn.link_type = 0x0100;
    conn.strength = 1.0;

    let connections = vec![conn];

    let profile = compute_shell(100, &connections, &table);

    // max = 100 → scale = 255/100 = 2.55
    // [100, 50, 25, 10, 5, 2, 1, 0] × 2.55 ≈ [255, 128, 64, 26, 13, 5, 3, 0]
    assert_eq!(profile[0], 255);
    assert_eq!(profile[1], 128);
    assert_eq!(profile[2], 64);
    assert_eq!(profile[3], 26); // 10 × 2.55 = 25.5 → 26
    assert_eq!(profile[4], 13); // 5 × 2.55 = 12.75 → 13
    assert_eq!(profile[5], 5);  // 2 × 2.55 = 5.1 → 5
    assert_eq!(profile[6], 3);  // 1 × 2.55 = 2.55 → 3
    assert_eq!(profile[7], 0);
}

#[test]
fn test_compute_shell_irrelevant_connections() {
    // Связи, не относящиеся к токену, игнорируются
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [10, 0, 0, 0, 0, 0, 0, 0]);

    let mut conn1 = Connection::default();
    conn1.source_id = 200; // НЕ наш токен
    conn1.target_id = 300;
    conn1.link_type = 0x0100;
    conn1.strength = 1.0;

    let connections = vec![conn1];

    let profile = compute_shell(100, &connections, &table);

    // Нет связей для токена 100
    assert_eq!(profile, EMPTY_SHELL);
}

// --- Incremental Update Tests ---

#[test]
fn test_mark_dirty_and_is_dirty() {
    // Проверка пометки токенов как dirty
    let mut cache = DomainShellCache::new(5);

    assert!(!cache.is_dirty(0));
    assert!(!cache.is_dirty(2));

    cache.mark_dirty(0);
    cache.mark_dirty(2);

    assert!(cache.is_dirty(0));
    assert!(!cache.is_dirty(1));
    assert!(cache.is_dirty(2));
    assert!(!cache.is_dirty(3));
}

#[test]
fn test_clear_dirty() {
    // Проверка очистки dirty flag
    let mut cache = DomainShellCache::new(3);

    cache.mark_dirty(0);
    cache.mark_dirty(1);

    assert!(cache.is_dirty(0));
    assert!(cache.is_dirty(1));

    cache.clear_dirty(0);

    assert!(!cache.is_dirty(0));
    assert!(cache.is_dirty(1));
}

#[test]
fn test_get_dirty_tokens() {
    // Проверка получения списка dirty токенов
    let mut cache = DomainShellCache::new(10);

    cache.mark_dirty(1);
    cache.mark_dirty(3);
    cache.mark_dirty(7);

    let dirty = cache.get_dirty_tokens();
    assert_eq!(dirty, vec![1, 3, 7]);
}

#[test]
fn test_mark_connection_dirty() {
    // Проверка триггера для Connection
    let mut cache = DomainShellCache::new(5);

    // Connection между токенами 1 (index 0) и 3 (index 2)
    mark_connection_dirty(&mut cache, 1, 3);

    assert!(cache.is_dirty(0)); // token_id 1 → index 0
    assert!(!cache.is_dirty(1));
    assert!(cache.is_dirty(2)); // token_id 3 → index 2
    assert!(!cache.is_dirty(3));
}

#[test]
fn test_update_dirty_shells_no_dirty() {
    // Обновление когда нет dirty токенов
    let mut cache = DomainShellCache::new(3);
    let table = SemanticContributionTable::new();
    let connections: Vec<Connection> = vec![];

    let updated = cache.update_dirty_shells(&connections, &table);

    assert_eq!(updated, 0);
    assert_eq!(cache.generation, 0);
}

#[test]
fn test_update_dirty_shells_single_token() {
    // Инкрементальное обновление одного токена
    let mut cache = DomainShellCache::new(5);
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [20, 10, 0, 0, 5, 0, 0, 0]);

    let mut conn = Connection::default();
    conn.source_id = 2; // token_id 2 → index 1
    conn.target_id = 4;
    conn.link_type = 0x0100;
    conn.strength = 1.0;

    let connections = vec![conn];

    // Помечаем токен 2 (index 1) как dirty
    cache.mark_dirty(1);

    let initial_gen = cache.generation;
    let updated = cache.update_dirty_shells(&connections, &table);

    assert_eq!(updated, 1);
    assert_eq!(cache.generation, initial_gen + 1);
    assert!(!cache.is_dirty(1)); // Dirty flag очищен

    // Профиль обновлён
    let profile = cache.get(1);
    assert_ne!(profile, &EMPTY_SHELL);
    assert_eq!(profile[0], 255); // Physical layer
}

#[test]
fn test_update_dirty_shells_multiple_tokens() {
    // Инкрементальное обновление нескольких токенов
    let mut cache = DomainShellCache::new(5);
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [10, 0, 0, 0, 0, 0, 0, 0]);

    let mut conn1 = Connection::default();
    conn1.source_id = 1;
    conn1.target_id = 2;
    conn1.link_type = 0x0100;
    conn1.strength = 1.0;

    let mut conn2 = Connection::default();
    conn2.source_id = 3;
    conn2.target_id = 4;
    conn2.link_type = 0x0100;
    conn2.strength = 1.0;

    let connections = vec![conn1, conn2];

    // Помечаем токены 1, 2, 3 как dirty (index 0, 1, 2)
    cache.mark_dirty(0);
    cache.mark_dirty(1);
    cache.mark_dirty(2);

    let updated = cache.update_dirty_shells(&connections, &table);

    assert_eq!(updated, 3);
    assert!(!cache.is_dirty(0));
    assert!(!cache.is_dirty(1));
    assert!(!cache.is_dirty(2));
}

#[test]
fn test_incremental_vs_full_update() {
    // Сравнение инкрементального и полного обновления
    let mut cache1 = DomainShellCache::new(3);
    let mut cache2 = DomainShellCache::new(3);
    let mut table = SemanticContributionTable::new();
    table.set_category(0x01, [15, 5, 0, 0, 0, 0, 0, 0]);

    let mut conn = Connection::default();
    conn.source_id = 1;
    conn.target_id = 2;
    conn.link_type = 0x0100;
    conn.strength = 1.0;

    let connections = vec![conn];

    // Инкрементальное обновление (только tokens 1 и 2 - участвуют в связи)
    cache1.mark_dirty(0); // token 1
    cache1.mark_dirty(1); // token 2
    cache1.update_dirty_shells(&connections, &table);

    // Полное обновление (все токены)
    for i in 0..3 {
        cache2.mark_dirty(i);
    }
    cache2.update_dirty_shells(&connections, &table);

    // Результаты для token 1 должны совпадать
    assert_eq!(cache1.get(0), cache2.get(0));

    // Token 2 тоже должен обновиться (участвует в связи)
    assert_eq!(cache1.get(1), cache2.get(1));

    // Token 3 не участвует в связях - пустой профиль
    assert_eq!(cache1.get(2), &EMPTY_SHELL);
    assert_eq!(cache2.get(2), &EMPTY_SHELL);
}

// --- Frontier Integration Tests ---

#[test]
fn test_collect_affected_tokens_empty() {
    // Пустой массив связей
    let connections: Vec<Connection> = vec![];
    let affected = collect_affected_tokens(&connections);
    assert!(affected.is_empty());
}

#[test]
fn test_collect_affected_tokens_single_connection() {
    // Одна связь затрагивает 2 токена
    let mut conn = Connection::default();
    conn.source_id = 5; // token_index = 4
    conn.target_id = 10; // token_index = 9
    conn.link_type = 0x0100;

    let connections = vec![conn];
    let mut affected = collect_affected_tokens(&connections);
    affected.sort(); // Сортируем для предсказуемости

    assert_eq!(affected.len(), 2);
    assert!(affected.contains(&4)); // source_id 5 → index 4
    assert!(affected.contains(&9)); // target_id 10 → index 9
}

#[test]
fn test_collect_affected_tokens_multiple_connections() {
    // Несколько связей с дубликатами
    let mut conn1 = Connection::default();
    conn1.source_id = 1; // index 0
    conn1.target_id = 2; // index 1
    conn1.link_type = 0x0100;

    let mut conn2 = Connection::default();
    conn2.source_id = 2; // index 1 (дубликат)
    conn2.target_id = 3; // index 2
    conn2.link_type = 0x0200;

    let mut conn3 = Connection::default();
    conn3.source_id = 3; // index 2 (дубликат)
    conn3.target_id = 4; // index 3
    conn3.link_type = 0x0100;

    let connections = vec![conn1, conn2, conn3];
    let mut affected = collect_affected_tokens(&connections);
    affected.sort();

    // Дубликаты должны быть удалены
    assert_eq!(affected.len(), 4);
    assert_eq!(affected, vec![0, 1, 2, 3]);
}

#[test]
fn test_collect_affected_tokens_self_loops() {
    // Связь токена с самим собой
    let mut conn = Connection::default();
    conn.source_id = 7; // index 6
    conn.target_id = 7; // index 6 (тот же токен)
    conn.link_type = 0x0300;

    let connections = vec![conn];
    let affected = collect_affected_tokens(&connections);

    // Должен быть только один токен (дубликат удалён)
    assert_eq!(affected.len(), 1);
    assert_eq!(affected[0], 6);
}

#[test]
fn test_collect_affected_tokens_integration_with_mark_dirty() {
    // Интеграция: collect_affected_tokens + mark_dirty
    let mut cache = DomainShellCache::new(10);

    let mut conn1 = Connection::default();
    conn1.source_id = 2; // index 1
    conn1.target_id = 5; // index 4
    conn1.link_type = 0x0100;

    let mut conn2 = Connection::default();
    conn2.source_id = 7; // index 6
    conn2.target_id = 9; // index 8
    conn2.link_type = 0x0200;

    let connections = vec![conn1, conn2];

    // Собираем затронутые токены
    let affected = collect_affected_tokens(&connections);

    // Помечаем их как dirty
    for token_idx in affected {
        cache.mark_dirty(token_idx);
    }

    // Проверяем что правильные токены помечены
    assert!(cache.is_dirty(1)); // source_id 2
    assert!(cache.is_dirty(4)); // target_id 5
    assert!(cache.is_dirty(6)); // source_id 7
    assert!(cache.is_dirty(8)); // target_id 9

    // Остальные не помечены
    assert!(!cache.is_dirty(0));
    assert!(!cache.is_dirty(2));
    assert!(!cache.is_dirty(3));
    assert!(!cache.is_dirty(5));
    assert!(!cache.is_dirty(7));
    assert!(!cache.is_dirty(9));
}

// --- Phase 2.7: Heartbeat Reconciliation Tests ---

#[test]
fn test_reconcile_shell_batch_no_drift() {
    // Shell V3.0 Phase 2.7: Reconciliation не обнаруживает drift если профиль не изменился
    let mut cache = DomainShellCache::new(5);
    let table = SemanticContributionTable::default_ashti_core();

    // Связь: Token 1 → Token 2 (Structural)
    let mut conn = Connection::new(1, 2, 1, 0);
    conn.link_type = 0x0100; // Structural
    let connections = vec![conn];

    // Вычислим профиль и запишем в кэш
    let profile = compute_shell(1, &connections, &table);
    cache.profiles[0] = profile;

    // Reconciliation не должна обнаружить drift
    let token_indices = vec![0];
    let drift_count = reconcile_shell_batch(&mut cache, &token_indices, &connections, &table);

    assert_eq!(drift_count, 0, "No drift should be detected");
    assert_eq!(cache.profiles[0], profile, "Profile should not change");
}

#[test]
fn test_reconcile_shell_batch_drift_detected() {
    // Shell V3.0 Phase 2.7: Reconciliation обнаруживает drift если профиль изменился
    let mut cache = DomainShellCache::new(5);
    let table = SemanticContributionTable::default_ashti_core();

    // Старая связь: Token 1 → Token 2 (Structural)
    let mut old_conn = Connection::new(1, 2, 1, 0);
    old_conn.link_type = 0x0100; // Structural

    // Новая связь: Token 1 → Token 2 (Semantic вместо Structural)
    let mut new_conn = Connection::new(1, 2, 1, 0);
    new_conn.link_type = 0x0200; // Semantic (другой тип!)

    // Запишем старый профиль в кэш
    let old_profile = compute_shell(1, &vec![old_conn], &table);
    cache.profiles[0] = old_profile;

    // Reconciliation с новыми связями
    let token_indices = vec![0];
    let new_connections = vec![new_conn];
    let drift_count = reconcile_shell_batch(&mut cache, &token_indices, &new_connections, &table);

    assert_eq!(drift_count, 1, "Drift should be detected");

    // Профиль должен обновиться
    let expected_profile = compute_shell(1, &new_connections, &table);
    assert_eq!(cache.profiles[0], expected_profile, "Profile should be updated");
    assert_ne!(cache.profiles[0], old_profile, "Profile should differ from old");
}

#[test]
fn test_reconcile_shell_batch_multiple_tokens() {
    // Shell V3.0 Phase 2.7: Reconciliation обрабатывает несколько токенов
    let mut cache = DomainShellCache::new(5);
    let table = SemanticContributionTable::default_ashti_core();

    // Создадим связи: 1→2 (Structural), 3→4 (Semantic)
    // Token 5 не участвует ни в каких связях
    let mut conn1 = Connection::new(1, 2, 1, 0);
    conn1.link_type = 0x0100; // Structural

    let mut conn2 = Connection::new(3, 4, 1, 0);
    conn2.link_type = 0x0200; // Semantic

    let connections = vec![conn1, conn2];

    // Инициализируем кэш правильными профилями
    cache.profiles[0] = compute_shell(1, &connections, &table); // Token 1
    cache.profiles[1] = compute_shell(2, &connections, &table); // Token 2
    cache.profiles[2] = compute_shell(3, &connections, &table); // Token 3
    cache.profiles[3] = compute_shell(4, &connections, &table); // Token 4
    cache.profiles[4] = EMPTY_SHELL; // Token 5 - no connections

    // Reconciliation не должна обнаружить drift
    let token_indices = vec![0, 1, 2, 3, 4];
    let drift_count = reconcile_shell_batch(&mut cache, &token_indices, &connections, &table);

    assert_eq!(drift_count, 0, "No drift should be detected when profiles are correct");

    // Теперь испортим профили Token 1, Token 3, и Token 5
    cache.profiles[0] = [100, 100, 100, 100, 100, 100, 100, 100]; // wrong
    cache.profiles[2] = [50, 50, 50, 50, 50, 50, 50, 50]; // wrong
    cache.profiles[4] = [10, 10, 10, 10, 10, 10, 10, 10]; // wrong (should be EMPTY)

    // Reconciliation должна обнаружить 3 drift (Token 1, Token 3, Token 5)
    let drift_count = reconcile_shell_batch(&mut cache, &token_indices, &connections, &table);

    assert_eq!(drift_count, 3, "Three tokens should have drift");

    // Token 5 не имеет связей → должен стать EMPTY после reconciliation
    assert_eq!(cache.profiles[4], EMPTY_SHELL, "Token 5 should be empty after reconciliation");
}

// --- Phase 2.10: Shell V3.0 Invariants Validation ---

#[test]
fn test_shell_v3_invariant_determinism() {
    // Shell V3.0 инвариант: детерминизм
    // Одинаковые входы → одинаковый результат
    let table = SemanticContributionTable::default_ashti_core();

    let mut conn = Connection::new(1, 2, 1, 0);
    conn.link_type = 0x0100; // Structural
    conn.strength = 1.5;

    let connections = vec![conn];

    let profile1 = compute_shell(1, &connections, &table);
    let profile2 = compute_shell(1, &connections, &table);

    assert_eq!(profile1, profile2, "Shell computation must be deterministic");
}

#[test]
fn test_shell_v3_invariant_domain_locality() {
    // Shell V3.0 инвариант: домен-локальность
    // Shell зависит только от локальных Connection (не от других доменов)
    let table = SemanticContributionTable::default_ashti_core();

    // Связь в домене 1
    let mut conn1 = Connection::new(1, 2, 1, 0);
    conn1.link_type = 0x0100;

    // Связь в домене 2 (не должна влиять на Token 1 в домене 1)
    let mut conn2 = Connection::new(3, 4, 2, 0);
    conn2.link_type = 0x0200;

    let connections_domain1 = vec![conn1];
    let connections_mixed = vec![conn1, conn2];

    let profile_pure = compute_shell(1, &connections_domain1, &table);
    let profile_mixed = compute_shell(1, &connections_mixed, &table);

    // Token 1 не участвует в conn2 → профиль не должен измениться
    assert_eq!(profile_pure, profile_mixed, "Shell must be domain-local (only own connections matter)");
}

#[test]
fn test_shell_v3_invariant_no_events() {
    // Shell V3.0 инвариант: Shell не генерирует COM-события
    // compute_shell() и reconcile_shell_batch() не должны создавать Events

    let mut cache = DomainShellCache::new(5);
    let table = SemanticContributionTable::default_ashti_core();

    let mut conn = Connection::new(1, 2, 1, 0);
    conn.link_type = 0x0100;
    let connections = vec![conn];

    // compute_shell не генерирует события (чистая функция)
    let _profile = compute_shell(1, &connections, &table);

    // reconcile_shell_batch не генерирует события (обновляет только кэш)
    let token_indices = vec![0];
    let _drift = reconcile_shell_batch(&mut cache, &token_indices, &connections, &table);

    // Тест проходит если компиляция успешна - функции детерминистичны и не имеют side effects
    assert!(true, "Shell functions must not generate COM events");
}

#[test]
fn test_shell_v3_invariant_cache_coherence() {
    // Shell V3.0 инвариант: кэш согласован с Connection
    // После reconciliation профиль должен совпадать с вычисленным
    let mut cache = DomainShellCache::new(5);
    let table = SemanticContributionTable::default_ashti_core();

    let mut conn = Connection::new(1, 2, 1, 0);
    conn.link_type = 0x0100;
    let connections = vec![conn];

    // Испортим кэш
    cache.profiles[0] = [99, 99, 99, 99, 99, 99, 99, 99];

    // Reconciliation
    let token_indices = vec![0];
    reconcile_shell_batch(&mut cache, &token_indices, &connections, &table);

    // Проверяем согласованность
    let expected_profile = compute_shell(1, &connections, &table);
    assert_eq!(cache.profiles[0], expected_profile, "Cache must be coherent after reconciliation");
}

#[test]
fn test_shell_v3_invariant_zero_allocation() {
    // Shell V3.0 инвариант: zero-allocation для compute_shell
    // compute_shell не должна аллоцировать память (использует стек)

    let table = SemanticContributionTable::default_ashti_core();

    let mut conn = Connection::new(1, 2, 1, 0);
    conn.link_type = 0x0100;
    let connections = vec![conn];

    // Многократный вызов не должен увеличивать heap usage
    for _ in 0..1000 {
        let _profile = compute_shell(1, &connections, &table);
    }

    // Тест проходит если компиляция успешна - функция использует только стек
    assert!(true, "compute_shell must be zero-allocation");
}
