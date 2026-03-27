// Тесты AshtiCore — 11-доменный фрактальный уровень Ashti_Core v2.0

use axiom_domain::AshtiCore;
use axiom_core::Token;

fn make_token(sutra_id: u32, mass: u8, temp: u8) -> Token {
    let mut t = Token::new(sutra_id, 1, [0, 0, 0], 1);
    t.mass = mass;
    t.temperature = temp;
    t
}

// --- Инициализация ---

#[test]
fn test_new_creates_ready_core() {
    let core = AshtiCore::new(1);
    assert!(core.is_ready(), "все 11 доменов должны быть зарегистрированы");
}

#[test]
fn test_level_id_preserved() {
    let core = AshtiCore::new(3);
    assert_eq!(core.level_id(), 3);
}

#[test]
fn test_two_cores_different_level_ids() {
    let c1 = AshtiCore::new(1);
    let c2 = AshtiCore::new(2);
    // level_id * 100 — пространства domain_id не пересекаются
    assert_ne!(c1.level_id(), c2.level_id());
}

// --- process() ---

#[test]
fn test_process_does_not_panic() {
    let mut core = AshtiCore::new(1);
    let token = make_token(42, 100, 128);
    let _ = core.process(token);
}

#[test]
fn test_slow_path_always_runs() {
    let mut core = AshtiCore::new(1);
    let token = make_token(1, 50, 100);
    let result = core.process(token);
    // ASHTI 1-8: ровно 8 результатов
    assert_eq!(result.slow_path.len(), 8, "медленный путь должен вернуть результаты всех 8 ASHTI");
}

#[test]
fn test_silence_no_reflex_on_empty_experience() {
    let mut core = AshtiCore::new(1);
    let token = make_token(7, 80, 120);
    let result = core.process(token);
    assert!(result.reflex.is_none(), "при пустой памяти рефлекса быть не должно");
}

#[test]
fn test_consolidated_result_present() {
    let mut core = AshtiCore::new(1);
    let token = make_token(5, 60, 90);
    let result = core.process(token);
    assert!(result.consolidated.is_some(), "MAYA должна вернуть консолидированный результат");
}

// --- Рефлекс после обучения ---

#[test]
fn test_reflex_after_training() {
    let mut core = AshtiCore::new(1);
    let token = make_token(10, 100, 200);

    // Добавляем след с высоким весом — выше порога рефлекса
    core.experience_mut().add_trace(token, 0.95, 1);

    let result = core.process(token);
    assert!(result.reflex.is_some(), "высокий weight должен дать рефлекс");
}

#[test]
fn test_no_reflex_with_low_weight() {
    let mut core = AshtiCore::new(1);
    let token = make_token(10, 100, 200);

    // Добавляем след с низким весом — ниже порога рефлекса
    core.experience_mut().add_trace(token, 0.1, 1);

    let result = core.process(token);
    assert!(result.reflex.is_none(), "низкий weight не должен давать рефлекс");
}

// --- apply_feedback ---

#[test]
fn test_apply_feedback_ok_for_existing_event() {
    let mut core = AshtiCore::new(1);
    let token = make_token(3, 70, 110);
    let result = core.process(token);

    // finalize_comparison для события, которое было создано при process
    let res = core.apply_feedback(result.event_id);
    assert!(res.is_ok(), "apply_feedback должен успешно завершиться для существующего event_id");
}

#[test]
fn test_apply_feedback_err_for_unknown_event() {
    let mut core = AshtiCore::new(1);
    let res = core.apply_feedback(9999);
    assert!(res.is_err(), "apply_feedback для несуществующего event_id должен вернуть Err");
}

// --- tick ---

#[test]
fn test_tick_does_not_panic() {
    let mut core = AshtiCore::new(1);
    for _ in 0..10 {
        core.tick();
    }
}

// --- state ---

#[test]
fn test_state_accessible_for_all_11_domains() {
    let core = AshtiCore::new(1);
    for i in 0..11 {
        assert!(core.state(i).is_some(), "state({i}) должен существовать");
    }
    assert!(core.state(11).is_none(), "state(11) должен быть None");
}
