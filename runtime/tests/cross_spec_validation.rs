// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// Cross-spec validation tests

use axiom_core::*;

#[test]
fn token_v5_1_structure_size() {
    assert_eq!(std::mem::size_of::<Token>(), 64); // Точно 64 байта!
    assert_eq!(std::mem::align_of::<Token>(), 64); // Cache-line aligned
}

#[test]
fn token_v5_1_field_offsets() {
    // Проверяем смещения полей согласно спецификации
    assert_eq!(std::mem::offset_of!(Token, sutra_id), 0);
    assert_eq!(std::mem::offset_of!(Token, domain_id), 4);
    assert_eq!(std::mem::offset_of!(Token, type_flags), 6);
    assert_eq!(std::mem::offset_of!(Token, position), 8);
    assert_eq!(std::mem::offset_of!(Token, velocity), 14);
    assert_eq!(std::mem::offset_of!(Token, target), 20);
    assert_eq!(std::mem::offset_of!(Token, valence), 28);
    assert_eq!(std::mem::offset_of!(Token, mass), 29);
    assert_eq!(std::mem::offset_of!(Token, temperature), 30);
    assert_eq!(std::mem::offset_of!(Token, state), 31);
    assert_eq!(std::mem::offset_of!(Token, lineage_hash), 32);
    assert_eq!(std::mem::offset_of!(Token, momentum), 40);
    assert_eq!(std::mem::offset_of!(Token, resonance), 52);
    assert_eq!(std::mem::offset_of!(Token, last_event_id), 56);
}

#[test]
fn connection_v5_0_structure_size() {
    assert_eq!(std::mem::size_of::<Connection>(), 64);
    assert_eq!(std::mem::align_of::<Connection>(), 64);
}

#[test]
fn connection_v5_0_field_offsets() {
    // Проверяем смещения полей согласно спецификации
    assert_eq!(std::mem::offset_of!(Connection, source_id), 0);
    assert_eq!(std::mem::offset_of!(Connection, target_id), 4);
    assert_eq!(std::mem::offset_of!(Connection, domain_id), 8);
    assert_eq!(std::mem::offset_of!(Connection, link_type), 10);
    assert_eq!(std::mem::offset_of!(Connection, flags), 12);
    assert_eq!(std::mem::offset_of!(Connection, strength), 16);
    assert_eq!(std::mem::offset_of!(Connection, current_stress), 20);
    assert_eq!(std::mem::offset_of!(Connection, ideal_dist), 24);
    assert_eq!(std::mem::offset_of!(Connection, elasticity), 28);
    assert_eq!(std::mem::offset_of!(Connection, density_gate), 32);
    assert_eq!(std::mem::offset_of!(Connection, thermal_gate), 33);
    assert_eq!(std::mem::offset_of!(Connection, created_at), 48);
    assert_eq!(std::mem::offset_of!(Connection, last_event_id), 56);
}

#[test]
fn com_v1_1_event_structure_size() {
    // COM V1.1: Event 64 байта, cache-line aligned
    assert_eq!(std::mem::size_of::<Event>(), 64);
    assert_eq!(std::mem::align_of::<Event>(), 64);
}

#[test]
fn com_v1_1_event_field_offsets() {
    // COM V1.1: Event 64 байта с pulse_id
    // Проверяем смещения полей согласно спецификации
    assert_eq!(std::mem::offset_of!(Event, event_id), 0);         // 8b
    assert_eq!(std::mem::offset_of!(Event, parent_event_id), 8);  // 8b
    assert_eq!(std::mem::offset_of!(Event, payload_hash), 16);    // 8b
    assert_eq!(std::mem::offset_of!(Event, target_id), 24);       // 4b
    assert_eq!(std::mem::offset_of!(Event, source_id), 28);       // 4b
    assert_eq!(std::mem::offset_of!(Event, domain_id), 32);       // 2b
    assert_eq!(std::mem::offset_of!(Event, event_type), 34);      // 2b
    assert_eq!(std::mem::offset_of!(Event, payload_size), 36);    // 2b
    assert_eq!(std::mem::offset_of!(Event, priority), 38);        // 1b
    assert_eq!(std::mem::offset_of!(Event, flags), 39);           // 1b
    assert_eq!(std::mem::offset_of!(Event, pulse_id), 40);        // 8b
}

#[test]
fn upo_v2_3_dynamic_trace_structure_size() {
    // UPO V2.3: DynamicTrace 32 байта с i16 координатами
    assert_eq!(std::mem::size_of::<DynamicTrace>(), 32);
    assert_eq!(std::mem::align_of::<DynamicTrace>(), 32);
}

#[test]
fn upo_v2_3_dynamic_trace_field_offsets() {
    // UPO V2.3: DynamicTrace 32 байта с i16 координатами
    // Проверяем смещения полей согласно спецификации
    assert_eq!(std::mem::offset_of!(DynamicTrace, last_update), 0);   // u64 - 8b
    assert_eq!(std::mem::offset_of!(DynamicTrace, weight), 8);        // f32 - 4b
    assert_eq!(std::mem::offset_of!(DynamicTrace, frequency), 12);    // f32 - 4b
    assert_eq!(std::mem::offset_of!(DynamicTrace, source_id), 16);    // u32 - 4b
    assert_eq!(std::mem::offset_of!(DynamicTrace, x), 20);            // i16 - 2b
    assert_eq!(std::mem::offset_of!(DynamicTrace, y), 22);            // i16 - 2b
    assert_eq!(std::mem::offset_of!(DynamicTrace, z), 24);            // i16 - 2b
    assert_eq!(std::mem::offset_of!(DynamicTrace, source_type), 26);  // u8  - 1b
    assert_eq!(std::mem::offset_of!(DynamicTrace, flags), 27);        // u8  - 1b
    assert_eq!(std::mem::offset_of!(DynamicTrace, resonance_class), 28); // u8 - 1b
}

#[test]
fn token_v5_1_invariants() {
    let mut token = Token::new(1, 1);
    token.last_event_id = 1;
    
    // Проверяем инварианты из спецификации
    assert!(token.validate());
    assert!(token.sutra_id > 0);
    assert!(token.domain_id > 0);
    assert!(token.mass > 0);
    assert!(token.last_event_id > 0);
    
    // Проверяем состояния
    assert!(token.is_active());
    token.state = STATE_SLEEPING;
    assert!(token.is_sleeping());
    token.state = STATE_LOCKED;
    assert!(token.is_locked());
}

#[test]
fn connection_v5_0_invariants() {
    let mut conn = Connection::new(1, 2, 1);
    conn.created_at = 1;
    conn.last_event_id = 1;
    
    // Проверяем инварианты из спецификации
    assert!(conn.validate());
    assert!(conn.source_id > 0);
    assert!(conn.target_id > 0);
    assert!(conn.domain_id > 0);
    assert!(conn.strength > 0.0);
    assert!(conn.current_stress >= 0.0);
    assert!(conn.elasticity > 0.0);
    assert!(conn.created_at > 0);
    assert!(conn.last_event_id >= conn.created_at);
    
    // Проверяем флаги
    conn.flags |= FLAG_CRITICAL;
    assert!(conn.is_critical());
    conn.flags |= FLAG_INHIBITED;
    assert!(conn.is_inhibited());
}

#[test]
fn com_v1_0_timeline_monotonicity() {
    let mut timeline = Timeline::new();
    
    let id1 = timeline.next_event_id(1);
    let id2 = timeline.next_event_id(1);
    let id3 = timeline.next_event_id(2);
    
    // Проверяем монотонность
    assert!(id1 < id2 && id2 < id3);
    assert_eq!(timeline.total_events, 3);
    
    // Проверяем смещения по доменам
    assert_eq!(timeline.domain_offsets[1], id2); // Последний event для домена 1
    assert_eq!(timeline.domain_offsets[2], id3); // Последний event для домена 2
}

#[test]
fn com_v1_0_event_validation() {
    let mut timeline = Timeline::new();
    let parent_id = timeline.next_event_id(1);
    let event_id = timeline.next_event_id(1);
    
    let event = Event::new(
        event_id,
        1,
        EventType::TokenCreate,
        EventPriority::Normal,
        12345,
        1,
        0,
        parent_id,
    );
    
    // Проверяем валидацию
    assert!(event.validate(&timeline));
    assert_eq!(event.event_id, event_id);
    assert_eq!(event.domain_id, 1);
    assert_eq!(event.event_type, EventType::TokenCreate as u16);
    assert_eq!(event.priority, EventPriority::Normal as u8);
    assert_eq!(event.payload_hash, 12345);
    assert_eq!(event.target_id, 1);
    assert_eq!(event.source_id, 0);
    assert_eq!(event.parent_event_id, parent_id);
}

#[test]
fn upo_v2_2_screen_octants() {
    let mut screen = Screen::new([100, 100, 100], 0.001, 0.01);
    
    // Проверяем октанты (0-7)
    let traces = [
        DynamicTrace::new(10, 10, 10, 1.0, 440.0, 1, TraceSourceType::Token, 1, 0), // +++
        DynamicTrace::new(-10, 10, 10, 1.0, 440.0, 1, TraceSourceType::Token, 2, 0), // -++
        DynamicTrace::new(10, -10, 10, 1.0, 440.0, 1, TraceSourceType::Token, 3, 0), // +-
        DynamicTrace::new(-10, -10, 10, 1.0, 440.0, 1, TraceSourceType::Token, 4, 0), // --
        DynamicTrace::new(10, 10, -10, 1.0, 440.0, 1, TraceSourceType::Token, 5, 0), // ++
        DynamicTrace::new(-10, 10, -10, 1.0, 440.0, 1, TraceSourceType::Token, 6, 0), // -+
        DynamicTrace::new(10, -10, -10, 1.0, 440.0, 1, TraceSourceType::Token, 7, 0), // +-
        DynamicTrace::new(-10, -10, -10, 1.0, 440.0, 1, TraceSourceType::Token, 8, 0), // --
    ];
    
    for trace in &traces {
        screen.write(trace);
    }
    
    // Проверяем что каждый октант имеет по одному следу
    for i in 0..8 {
        assert_eq!(screen.octants[i].trace_count, 1);
        assert_eq!(screen.octants[i].total_energy, 1.0);
        assert_eq!(screen.octants[i].dominant_frequency, 440.0);
        assert_eq!(screen.octants[i].last_event_id, 1);
    }
    
    assert_eq!(screen.trace_count, 8);
    assert_eq!(screen.total_energy, 8.0);
}

#[test]
fn upo_v2_2_decay_and_eternal_memory() {
    let mut screen = Screen::new([100, 100, 100], 0.001, 0.1);
    
    let trace = DynamicTrace::new(0, 0, 0, 1.0, 440.0, 1, TraceSourceType::Token, 1, 0);
    screen.write(&trace);
    
    // Применяем затухание
    screen.set_current_event(10);
    
    // Проверяем что вес уменьшился но не ниже min_intensity
    assert!(screen.traces[0].weight < 1.0);
    assert!(screen.traces[0].weight >= screen.min_intensity);
    
    // Применяем сильное затухание
    screen.set_current_event(100);
    
    // Проверяем вечную память
    assert!(screen.traces[0].weight == screen.min_intensity);
    assert!(screen.traces[0].is_eternal());
}

#[test]
fn integration_com_with_token_and_connection() {
    let mut timeline = Timeline::new();
    
    // Создаем токен
    let mut token = Token::new(1, 1);
    token.last_event_id = timeline.next_event_id(1);
    
    // Создаем связь
    let mut conn = Connection::new(1, 2, 1);
    conn.created_at = timeline.next_event_id(1);
    conn.last_event_id = timeline.next_event_id(1);
    
    // Проверяем COM синхронизацию
    assert!(token.last_event_id < conn.last_event_id);
    assert!(conn.last_event_id == timeline.current_event_id);
    
    // Создаем событие обновления токена
    let update_event = Event::new(
        timeline.next_event_id(1),
        1,
        EventType::TokenUpdate,
        EventPriority::Normal,
        54321,
        token.sutra_id,
        0,
        token.last_event_id,
    );
    
    assert!(update_event.validate(&timeline));
    assert!(update_event.parent_event_id == token.last_event_id);
}
