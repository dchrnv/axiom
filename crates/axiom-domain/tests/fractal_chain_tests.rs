// Этап 12A — FractalChain: иерархия уровней AshtiCore

use axiom_core::Token;
use axiom_domain::{AshtiCore, FractalChain};

fn make_token(id: u32) -> Token {
    let mut t = Token::new(id, 1, [0, 0, 0], 1);
    t.mass = 10;
    t.temperature = 100;
    t
}

// ─── AshtiCore: take_maya_output / set_sutra_input ────────────────────────────

#[test]
fn test_take_maya_output_empty() {
    let mut core = AshtiCore::new(0);
    assert!(core.take_maya_output().is_none());
}

#[test]
fn test_inject_and_take_maya() {
    let mut core = AshtiCore::new(0);
    let maya_id = 10u16; // 0 * 100 + 10
    let token = make_token(1);
    core.inject_token(maya_id, token).unwrap();
    let out = core.take_maya_output();
    assert!(out.is_some());
    assert_eq!(core.token_count(maya_id), 0);
}

#[test]
fn test_set_sutra_input() {
    let mut core = AshtiCore::new(1);
    let sutra_id = 100u16; // 1 * 100
    let token = make_token(1);
    core.set_sutra_input(token).unwrap();
    assert_eq!(core.token_count(sutra_id), 1);
}

#[test]
fn test_set_sutra_input_level2() {
    let mut core = AshtiCore::new(2);
    let sutra_id = 2u16 * 100; // = 200
    core.set_sutra_input(make_token(5)).unwrap();
    assert_eq!(core.token_count(sutra_id), 1);
}

// ─── FractalChain ──────────────────────────────────────────────────────────────

#[test]
fn test_chain_new_depth() {
    let chain = FractalChain::new(3);
    assert_eq!(chain.depth(), 3);
}

#[test]
fn test_chain_level_access() {
    let chain = FractalChain::new(2);
    assert!(chain.level(0).is_some());
    assert!(chain.level(1).is_some());
    assert!(chain.level(2).is_none());
}

#[test]
fn test_chain_inject_input() {
    let mut chain = FractalChain::new(2);
    chain.inject_input(make_token(1)).unwrap();
    // SUTRA of level 0 has domain_id = 0
    assert_eq!(chain.level(0).unwrap().token_count(0), 1);
}

#[test]
fn test_chain_take_output_empty() {
    let mut chain = FractalChain::new(2);
    assert!(chain.take_output().is_none());
}

#[test]
fn test_chain_take_output() {
    let mut chain = FractalChain::new(2);
    // Вручную положить токен в MAYA последнего уровня (уровень 1, MAYA = 110)
    chain
        .level_mut(1)
        .unwrap()
        .inject_token(110, make_token(42))
        .unwrap();
    let out = chain.take_output();
    assert!(out.is_some());
}

#[test]
fn test_chain_tick_returns_events() {
    let mut chain = FractalChain::new(2);
    // Тик без токенов не должен паниковать
    let events = chain.tick();
    // Может быть пустым, главное не паника
    let _ = events;
}

#[test]
fn test_chain_maya_to_sutra_propagation() {
    let mut chain = FractalChain::new(2);

    // Положить токен в MAYA уровня 0 (domain_id = 10)
    let token = make_token(7);
    chain.level_mut(0).unwrap().inject_token(10, token).unwrap();

    // tick(): MAYA(0) → SUTRA(1)
    chain.tick();

    // После тика MAYA уровня 0 должна быть пуста (токен передан)
    assert_eq!(chain.level(0).unwrap().token_count(10), 0);
    // SUTRA уровня 1 (domain_id = 100) должна содержать токен
    assert_eq!(chain.level(1).unwrap().token_count(100), 1);
}

#[test]
fn test_chain_exchange_skills_no_panic() {
    let mut chain = FractalChain::new(3);
    // Без навыков — нет паники, возвращает 0
    let count = chain.exchange_skills();
    assert_eq!(count, 0);
}
