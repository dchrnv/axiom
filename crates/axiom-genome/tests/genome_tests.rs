use std::path::Path;
use axiom_genome::{
    Genome, GenomeError, GenomeIndex,
    ModuleId, ResourceId, Permission, DataType,
};

// ============================================================================
// default_ashti_core + validate
// ============================================================================

#[test]
fn test_default_genome_is_valid() {
    let genome = Genome::default_ashti_core();
    assert!(genome.validate().is_ok());
}

#[test]
fn test_default_genome_version() {
    let genome = Genome::default_ashti_core();
    assert_eq!(genome.version, 1);
}

#[test]
fn test_default_genome_invariants() {
    let genome = Genome::default_ashti_core();
    let inv = &genome.invariants;
    assert_eq!(inv.token_size, 64);
    assert_eq!(inv.connection_size, 64);
    assert_eq!(inv.event_size, 32);
    assert_eq!(inv.domain_config_size, 128);
    assert_eq!(inv.max_domains, 11);
    assert!(inv.sutra_write_exclusive);
    assert!(inv.no_wall_clock_in_core);
    assert!(inv.event_id_monotonic);
}

#[test]
fn test_default_genome_config() {
    let genome = Genome::default_ashti_core();
    assert_eq!(genome.config.ashti_domain_count, 11);
    assert_eq!(genome.config.default_heartbeat_interval, 1024);
    assert!(genome.config.arbiter_response_timeout > 0);
}

#[test]
fn test_default_genome_has_access_rules() {
    let genome = Genome::default_ashti_core();
    assert!(!genome.access_rules.is_empty());
}

#[test]
fn test_default_genome_has_protocol_rules() {
    let genome = Genome::default_ashti_core();
    assert!(!genome.protocol_rules.is_empty());
}

// ============================================================================
// validate — негативные случаи
// ============================================================================

#[test]
fn test_invalid_token_size_fails_validation() {
    let mut genome = Genome::default_ashti_core();
    genome.invariants.token_size = 32; // неверно
    assert_eq!(
        genome.validate(),
        Err(GenomeError::InvariantViolation("token_size must be 64"))
    );
}

#[test]
fn test_invalid_event_size_fails_validation() {
    let mut genome = Genome::default_ashti_core();
    genome.invariants.event_size = 64; // неверно
    assert!(genome.validate().is_err());
}

#[test]
fn test_missing_guardian_codex_access_fails() {
    let mut genome = Genome::default_ashti_core();
    genome.access_rules.retain(|r|
        !(r.module == ModuleId::Guardian && r.resource == ResourceId::CodexRules)
    );
    assert_eq!(genome.validate(), Err(GenomeError::MissingGuardianAccess));
}

#[test]
fn test_missing_mandatory_protocol_fails() {
    let mut genome = Genome::default_ashti_core();
    // Удаляем обязательный маршрут SUTRA→EXPERIENCE
    genome.protocol_rules.retain(|r|
        !(r.source == ModuleId::Sutra && r.target == ModuleId::Experience && r.mandatory)
    );
    assert!(matches!(
        genome.validate(),
        Err(GenomeError::MissingMandatoryProtocol(_))
    ));
}

#[test]
fn test_invalid_domain_count_fails() {
    let mut genome = Genome::default_ashti_core();
    genome.config.ashti_domain_count = 8;
    assert!(genome.validate().is_err());
}

#[test]
fn test_zero_heartbeat_interval_fails() {
    let mut genome = Genome::default_ashti_core();
    genome.config.default_heartbeat_interval = 0;
    assert!(genome.validate().is_err());
}

// ============================================================================
// GenomeIndex — O(1) lookup
// ============================================================================

#[test]
fn test_index_guardian_has_readwrite_on_codex() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    assert!(index.check_access(ModuleId::Guardian, ResourceId::CodexRules, Permission::ReadWrite));
}

#[test]
fn test_index_guardian_can_read_genome() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    assert!(index.check_access(ModuleId::Guardian, ResourceId::GenomeConfig, Permission::Read));
}

#[test]
fn test_index_adapters_cannot_write_codex() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    assert!(!index.check_access(ModuleId::Adapters, ResourceId::CodexRules, Permission::ReadWrite));
}

#[test]
fn test_index_arbiter_cannot_control_experience() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    // Arbiter имеет только Read на ExperienceMemory — Control запрещён
    assert!(!index.check_access(ModuleId::Arbiter, ResourceId::ExperienceMemory, Permission::Control));
}

#[test]
fn test_index_protocol_sutra_to_experience_exists() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    assert!(index.check_protocol(ModuleId::Sutra, ModuleId::Experience));
}

#[test]
fn test_index_protocol_experience_to_arbiter_exists() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    assert!(index.check_protocol(ModuleId::Experience, ModuleId::Arbiter));
}

#[test]
fn test_index_protocol_adapters_to_sutra_forbidden() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    // Адаптеры не имеют маршрута в SUTRA
    assert!(!index.check_protocol(ModuleId::Adapters, ModuleId::Sutra));
}

#[test]
fn test_index_protocol_typed_token_reference() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    assert!(index.check_protocol_typed(
        &genome,
        ModuleId::Sutra,
        ModuleId::Experience,
        DataType::TokenReference,
    ));
}

#[test]
fn test_index_protocol_typed_wrong_type_rejected() {
    let genome = Genome::default_ashti_core();
    let index = GenomeIndex::build(&genome);
    // SUTRA→EXPERIENCE не передаёт Feedback
    assert!(!index.check_protocol_typed(
        &genome,
        ModuleId::Sutra,
        ModuleId::Experience,
        DataType::Feedback,
    ));
}

// ============================================================================
// from_yaml (Фаза B)
// ============================================================================

/// Путь к config/genome.yaml относительно workspace root.
/// В тестах используем CARGO_MANIFEST_DIR чтобы найти workspace root.
fn genome_yaml_path() -> std::path::PathBuf {
    // axiom-genome crate находится в crates/axiom-genome/
    // workspace root — на два уровня выше
    let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    Path::new(&manifest).join("../../config/genome.yaml")
}

#[test]
fn test_from_yaml_valid() {
    let path = genome_yaml_path();
    let genome = Genome::from_yaml(&path).expect("config/genome.yaml should parse and validate");
    assert_eq!(genome.version, 1);
    assert!(genome.validate().is_ok());
}

#[test]
fn test_from_yaml_matches_default() {
    let path = genome_yaml_path();
    let yaml_genome = Genome::from_yaml(&path).unwrap();
    let default_genome = Genome::default_ashti_core();

    // Инварианты совпадают
    assert_eq!(yaml_genome.invariants.token_size, default_genome.invariants.token_size);
    assert_eq!(yaml_genome.invariants.max_domains, default_genome.invariants.max_domains);
    assert_eq!(yaml_genome.invariants.no_wall_clock_in_core, default_genome.invariants.no_wall_clock_in_core);

    // Количество правил совпадает
    assert_eq!(yaml_genome.access_rules.len(), default_genome.access_rules.len());
    assert_eq!(yaml_genome.protocol_rules.len(), default_genome.protocol_rules.len());

    // Config совпадает
    assert_eq!(yaml_genome.config.ashti_domain_count, default_genome.config.ashti_domain_count);
    assert_eq!(yaml_genome.config.default_heartbeat_interval, default_genome.config.default_heartbeat_interval);
}

#[test]
fn test_from_yaml_index_same_behavior() {
    let path = genome_yaml_path();
    let yaml_genome = Genome::from_yaml(&path).unwrap();
    let yaml_index = GenomeIndex::build(&yaml_genome);

    let default_genome = Genome::default_ashti_core();
    let default_index = GenomeIndex::build(&default_genome);

    // O(1) lookup должны давать одинаковые результаты
    assert_eq!(
        yaml_index.check_access(ModuleId::Guardian, ResourceId::CodexRules, Permission::ReadWrite),
        default_index.check_access(ModuleId::Guardian, ResourceId::CodexRules, Permission::ReadWrite),
    );
    assert_eq!(
        yaml_index.check_protocol(ModuleId::Sutra, ModuleId::Experience),
        default_index.check_protocol(ModuleId::Sutra, ModuleId::Experience),
    );
    assert_eq!(
        yaml_index.check_access(ModuleId::Adapters, ResourceId::CodexRules, Permission::ReadWrite),
        default_index.check_access(ModuleId::Adapters, ResourceId::CodexRules, Permission::ReadWrite),
    );
}

#[test]
fn test_from_yaml_missing_file_error() {
    let result = Genome::from_yaml(Path::new("/nonexistent/path/genome.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_from_yaml_invalid_content_error() {
    // Пишем невалидный YAML во временный файл
    let tmp = std::env::temp_dir().join("invalid_genome_test.yaml");
    std::fs::write(&tmp, "not: valid: genome: yaml: !!!").unwrap();
    let result = Genome::from_yaml(&tmp);
    // Может быть либо ParseError либо InvariantViolation — в любом случае Err
    assert!(result.is_err());
    let _ = std::fs::remove_file(&tmp);
}
