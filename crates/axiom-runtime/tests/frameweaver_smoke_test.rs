// FrameWeaver V1.1 — End-to-End Smoke Test (Этап 4 стабилизации)
//
// Сценарий:
//   1. Создать синтаксический узор в MAYA через UCL
//   2. Прогнать 25 тиков (FrameWeaver сканирует на каждом при scan_interval=1)
//   3. Проверить: в EXPERIENCE появился Frame-анкер
//   4. Послать UCL UnfoldFrame в LOGIC
//   5. Проверить: в LOGIC появилась копия Frame
//   6. Проверить метрики FrameWeaverStats

use axiom_core::{Connection, FLAG_ACTIVE, TOKEN_FLAG_FRAME_ANCHOR};
use axiom_runtime::{AxiomEngine, FrameWeaver, FrameWeaverConfig};
use axiom_ucl::{OpCode, UclCommand, UnfoldFramePayload};

const MAYA_ID: u16 = 110;
const EXPERIENCE_ID: u16 = 109;
const LOGIC_ID: u16 = 106;

fn syn_conn(source: u32, target: u32, layer: u8) -> Connection {
    let mut c = Connection::new(source, target, MAYA_ID, 1);
    c.link_type = 0x0800 | ((layer as u16) << 4);
    c.flags = FLAG_ACTIVE;
    c
}

fn tick(engine: &mut AxiomEngine) {
    engine.process_command(&UclCommand::new(OpCode::TickForward, 0, 100, 0));
}

#[test]
fn frameweaver_end_to_end_smoke() {
    // Настроить engine с агрессивным FrameWeaver (scan каждый тик, threshold=3)
    let mut engine = AxiomEngine::new();
    engine.frame_weaver = FrameWeaver::new(FrameWeaverConfig {
        scan_interval_ticks: 1,
        stability_threshold: 3,
        min_participants: 2,
        ..Default::default()
    });

    // Шаг 1: Создать синтаксический узор в MAYA
    // head=10, targets=20(layer1) и 30(layer2)
    engine
        .ashti
        .inject_connection(MAYA_ID, syn_conn(10, 20, 1))
        .unwrap();
    engine
        .ashti
        .inject_connection(MAYA_ID, syn_conn(10, 30, 2))
        .unwrap();

    // Шаг 2: Прогнать тики — FrameWeaver сканирует и кристаллизует при stability >= 3
    for _ in 0..25 {
        tick(&mut engine);
    }

    // Шаг 3: В EXPERIENCE должен появиться Frame-анкер
    let exp_idx = engine
        .ashti
        .index_of(EXPERIENCE_ID)
        .expect("EXPERIENCE domain must exist");
    let exp_state = engine
        .ashti
        .state(exp_idx)
        .expect("EXPERIENCE state must be accessible");

    let frame_anchors: Vec<_> = exp_state
        .tokens
        .iter()
        .filter(|t| (t.type_flags & TOKEN_FLAG_FRAME_ANCHOR) != 0)
        .collect();

    assert!(
        !frame_anchors.is_empty(),
        "EXPERIENCE должен содержать Frame-анкер после {} тиков; scans_performed={}",
        25,
        engine.frame_weaver.stats.scans_performed,
    );

    let anchor = frame_anchors[0];
    let anchor_id = anchor.sutra_id;

    // Убедиться что в EXPERIENCE есть связи от анкера к участникам
    let exp_bonds: Vec<_> = exp_state
        .connections
        .iter()
        .filter(|c| c.source_id == anchor_id)
        .collect();
    assert!(
        !exp_bonds.is_empty(),
        "Frame-анкер должен иметь BondTokens в EXPERIENCE (restore требует связи)"
    );

    // Шаг 4: Участники должны быть в EXPERIENCE для успеха UnfoldFrame
    // (restore_frame_from_anchor ищет target_id в source_state.tokens)
    // Для простоты smoke-теста добавляем participant-токены прямо в EXPERIENCE
    for &target_sutra_id in &[10u32, 20u32, 30u32] {
        if engine
            .ashti
            .find_token_by_sutra_id(EXPERIENCE_ID, target_sutra_id)
            .is_none()
        {
            engine
                .ashti
                .inject_token(
                    EXPERIENCE_ID,
                    axiom_core::Token::new(target_sutra_id, EXPERIENCE_ID, [0; 3], 0),
                )
                .ok();
        }
    }

    // Шаг 5: Послать UCL UnfoldFrame в LOGIC
    let logic_tokens_before = engine.ashti.token_count(LOGIC_ID);
    let logic_idx = engine.ashti.index_of(LOGIC_ID).unwrap();
    let logic_conns_before = engine.ashti.state(logic_idx).unwrap().connections.len();

    let unfold_payload = UnfoldFramePayload {
        frame_anchor_id: anchor_id,
        target_domain_id: LOGIC_ID,
        unfold_depth: 1,
        reserved: [0; 41],
    };
    let unfold_cmd = UclCommand::new(OpCode::UnfoldFrame, 0, 10, 0).with_payload(&unfold_payload);
    let unfold_result = engine.process_command(&unfold_cmd);

    assert_eq!(
        unfold_result.status,
        axiom_ucl::CommandStatus::Success as u8,
        "UnfoldFrame должен завершиться успешно"
    );

    // Шаг 6: Проверить: в LOGIC появился токен-анкер и связи
    assert_eq!(
        engine.ashti.token_count(LOGIC_ID),
        logic_tokens_before + 1,
        "В LOGIC должен появиться один новый токен (копия Frame-анкера)"
    );
    let logic_conns_after = engine
        .ashti
        .state(engine.ashti.index_of(LOGIC_ID).unwrap())
        .unwrap()
        .connections
        .len();
    assert!(
        logic_conns_after > logic_conns_before,
        "В LOGIC должны появиться связи от развёрнутого Frame"
    );

    // Шаг 7: Проверить метрики FrameWeaverStats
    assert_eq!(
        engine.frame_weaver.stats.scans_performed, 25,
        "должно быть 25 сканов"
    );
    assert!(
        engine.frame_weaver.stats.candidates_detected >= 1,
        "хотя бы 1 кандидат обнаружен"
    );
    assert!(
        engine.frame_weaver.stats.crystallizations_approved >= 1,
        "хотя бы 1 кристаллизация"
    );
    assert!(
        engine.frame_weaver.stats.frames_in_experience >= 1,
        "Frame виден в stats"
    );
    assert_eq!(
        engine.frame_weaver.stats.unfold_requests, 1,
        "1 UnfoldFrame запрос"
    );

    // Шаг 8: Оригинальный Frame в EXPERIENCE остался нетронутым
    assert!(
        engine
            .ashti
            .find_token_by_sutra_id(EXPERIENCE_ID, anchor_id)
            .is_some(),
        "Оригинальный Frame-анкер в EXPERIENCE не должен быть удалён"
    );
}
