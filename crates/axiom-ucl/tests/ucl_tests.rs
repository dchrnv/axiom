use axiom_ucl::*;

#[test]
fn test_ucl_command_size() {
    assert_eq!(std::mem::size_of::<UclCommand>(), 64);
    assert_eq!(std::mem::align_of::<UclCommand>(), 64);
}

#[test]
fn test_ucl_result_size() {
    assert_eq!(std::mem::size_of::<UclResult>(), 32);
    assert_eq!(std::mem::align_of::<UclResult>(), 32);
}

#[test]
fn test_spawn_domain_command() {
    let command = UclBuilder::spawn_domain(123, 1); // SUTRA

    assert_eq!(command.opcode, OpCode::SpawnDomain as u16);
    assert_eq!(command.target_id, 123);
    assert!(command.is_valid());

    let payload = command.get_payload::<SpawnDomainPayload>();
    assert_eq!(payload.factory_preset, 1);
    assert_eq!(payload.structural_role, 0); // preset=1 (Sutra) → StructuralRole::Sutra=0
}

#[test]
fn test_apply_force_command() {
    let force = [1.0, 0.0, 0.0];
    let command = UclBuilder::apply_force(456, force, 10.0);

    assert_eq!(command.opcode, OpCode::ApplyForce as u16);
    assert_eq!(command.target_id, 456);
    assert!(command.is_valid());

    let payload = command.get_payload::<ApplyForcePayload>();
    assert_eq!(payload.force_vector, force);
    assert_eq!(payload.magnitude, 10.0);
}

#[test]
fn test_ucl_result() {
    let success = UclResult::success(789);
    assert!(success.is_success());
    assert_eq!(success.command_id, 789);

    let error = UclResult::error(789, CommandStatus::PhysicsViolation, 1001);
    assert!(!error.is_success());
    assert_eq!(error.status, CommandStatus::PhysicsViolation as u8);
    assert_eq!(error.error_code, 1001);
}

// ─── ucl_preset_to_structural_role (D-08) ────────────────────────────────────

#[test]
fn test_preset_void_maps_to_structural_role_8() {
    assert_eq!(ucl_preset_to_structural_role(0), 8, "UCL Void(0) → StructuralRole::Void(8)");
}

#[test]
fn test_preset_sutra_maps_to_structural_role_0() {
    assert_eq!(ucl_preset_to_structural_role(1), 0, "UCL Sutra(1) → StructuralRole::Sutra(0)");
}

#[test]
fn test_preset_others_pass_through() {
    for n in [2u8, 3, 4, 5, 6, 7, 9, 10] {
        assert_eq!(ucl_preset_to_structural_role(n), n, "preset {n} должен проходить без изменений");
    }
}

#[test]
fn test_spawn_domain_builder_structural_role_correct() {
    let cmd = UclBuilder::spawn_domain(0, 0); // preset=0 (Void)
    let payload = cmd.payload;
    // factory_preset = payload[2], structural_role = payload[3]
    assert_eq!(payload[2], 0, "factory_preset должен остаться 0 (Void)");
    assert_eq!(payload[3], 8, "structural_role должен быть 8 (StructuralRole::Void)");

    let cmd2 = UclBuilder::spawn_domain(0, 1); // preset=1 (Sutra)
    assert_eq!(cmd2.payload[2], 1, "factory_preset должен остаться 1 (Sutra)");
    assert_eq!(cmd2.payload[3], 0, "structural_role должен быть 0 (StructuralRole::Sutra)");
}
