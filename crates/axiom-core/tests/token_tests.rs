use axiom_core::{Token, STATE_ACTIVE, STATE_LOCKED, STATE_SLEEPING};

#[test]
fn test_token_size() {
    assert_eq!(std::mem::size_of::<Token>(), 64);
    assert_eq!(std::mem::align_of::<Token>(), 64);
}

#[test]
fn test_token_new() {
    let token = Token::new(1, 1, [0, 0, 0], 100);
    assert_eq!(token.sutra_id, 1);
    assert_eq!(token.domain_id, 1);
    assert_eq!(token.position, [0, 0, 0]);
    assert_eq!(token.last_event_id, 100);
    assert!(token.validate().is_ok());
}

#[test]
fn test_token_validation() {
    let mut token = Token::new(1, 1, [0, 0, 0], 100);
    assert!(token.validate().is_ok());

    token.sutra_id = 0;
    assert!(token.validate().is_err());
    token.sutra_id = 1;

    token.domain_id = 0;
    assert!(token.validate().is_err());
    token.domain_id = 1;

    token.mass = 0;
    assert!(token.validate().is_err());
    token.mass = 100;

    token.last_event_id = 0;
    assert!(token.validate().is_err());
}

#[test]
fn test_token_state_flags() {
    let mut token = Token::new(1, 1, [0, 0, 0], 100);

    token.state = STATE_ACTIVE;
    assert!(token.is_active());
    assert!(!token.is_sleeping());
    assert!(!token.is_locked());

    token.state = STATE_SLEEPING;
    assert!(!token.is_active());
    assert!(token.is_sleeping());
    assert!(!token.is_locked());

    token.state = STATE_LOCKED;
    assert!(!token.is_active());
    assert!(!token.is_sleeping());
    assert!(token.is_locked());
}

#[test]
fn test_update_momentum() {
    let mut token = Token::new(1, 1, [0, 0, 0], 100);

    // Применяем силу
    let force = [10, -5, 3];
    token.update_momentum(force, 101);

    assert_eq!(token.momentum[0], 10);
    assert_eq!(token.momentum[1], -5);
    assert_eq!(token.momentum[2], 3);
    assert_eq!(token.last_event_id, 101);

    // Применяем ещё раз - импульс накапливается
    token.update_momentum([5, 5, -3], 102);
    assert_eq!(token.momentum[0], 15);
    assert_eq!(token.momentum[1], 0);
    assert_eq!(token.momentum[2], 0);
}

#[test]
fn test_compute_resonance() {
    let mut token1 = Token::new(1, 1, [0, 0, 0], 100);
    token1.valence = 4; // Установим валентность
    let mut token2 = Token::new(2, 1, [10, 0, 0], 101);
    token2.valence = 4;

    // Одинаковые параметры, близко → высокий резонанс
    let resonance = token1.compute_resonance(&token1);
    assert!(resonance > 70, "resonance = {}", resonance);

    // Близко, похожие параметры → средний/высокий резонанс
    let resonance = token1.compute_resonance(&token2);
    assert!(resonance > 40, "resonance = {}", resonance);

    // Далеко → более низкий резонанс
    token2.position = [200, 200, 200];
    let resonance = token1.compute_resonance(&token2);
    assert!(resonance < 60, "resonance = {}", resonance);
}
