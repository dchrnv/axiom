// Этап 9C — Config hot reload: ConfigWatcher
use axiom_config::ConfigWatcher;
use std::time::Duration;

// Шаблон минимального axiom.yaml для тестов
const AXIOM_YAML_TEMPLATE: &str = r#"
runtime:
  file: "x"
  schema: "y"
schema:
  domain: "a"
  token: "b"
  connection: "c"
  grid: "d"
  upo: "e"
loader:
  format: yaml
  validation: strict
  cache_enabled: false
"#;

const AXIOM_YAML_MODIFIED: &str = r#"
runtime:
  file: "x2"
  schema: "y2"
schema:
  domain: "a"
  token: "b"
  connection: "c"
  grid: "d"
  upo: "e"
loader:
  format: yaml
  validation: relaxed
  cache_enabled: true
"#;

fn temp_dir(suffix: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("axiom_watcher_{suffix}"))
}

// ─── ConfigWatcher::new ───────────────────────────────────────────────────────

#[test]
fn test_watcher_new_on_existing_file() {
    let dir = temp_dir("new");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_TEMPLATE).unwrap();

    let watcher = ConfigWatcher::new(&path);
    assert!(watcher.is_ok(), "ConfigWatcher::new failed: {:?}", watcher.err());
    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn test_watcher_config_path() {
    let dir = temp_dir("path");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_TEMPLATE).unwrap();

    let watcher = ConfigWatcher::new(&path).unwrap();
    assert_eq!(watcher.config_path(), path.as_path());
    std::fs::remove_dir_all(dir).ok();
}

// ─── poll — no change ────────────────────────────────────────────────────────

#[test]
fn test_watcher_poll_no_change_returns_none() {
    let dir = temp_dir("nochange");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_TEMPLATE).unwrap();

    let watcher = ConfigWatcher::new(&path).unwrap();
    // No file modification — poll must return None
    assert!(watcher.poll().is_none());
    std::fs::remove_dir_all(dir).ok();
}

// ─── poll — file changed ──────────────────────────────────────────────────────

#[test]
fn test_watcher_poll_detects_file_change() {
    let dir = temp_dir("change");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_TEMPLATE).unwrap();

    let watcher = ConfigWatcher::new(&path).unwrap();
    // Drain initial events (inotify может эмитить событие при watch)
    std::thread::sleep(Duration::from_millis(50));
    let _ = watcher.poll();

    // Модифицируем файл
    std::fs::write(&path, AXIOM_YAML_MODIFIED).unwrap();
    std::thread::sleep(Duration::from_millis(150)); // ждём inotify

    let result = watcher.poll();
    assert!(result.is_some(), "poll() должен вернуть новую конфигурацию после изменения файла");

    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn test_watcher_reload_reflects_new_values() {
    let dir = temp_dir("values");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_TEMPLATE).unwrap();

    let watcher = ConfigWatcher::new(&path).unwrap();
    std::thread::sleep(Duration::from_millis(50));
    let _ = watcher.poll();

    // Записываем изменённый файл
    std::fs::write(&path, AXIOM_YAML_MODIFIED).unwrap();
    std::thread::sleep(Duration::from_millis(150));

    let cfg = watcher.poll().expect("expected new config");
    assert!(cfg.root.loader.cache_enabled, "cache_enabled должен стать true");
    assert_eq!(cfg.root.loader.validation, "relaxed", "validation должен обновиться");

    std::fs::remove_dir_all(dir).ok();
}

// ─── GENOME-инвариант ─────────────────────────────────────────────────────────

#[test]
fn test_watcher_genome_file_change_not_detected() {
    // Watcher следит за axiom.yaml.
    // Изменение genome.yaml в другом месте не вызывает poll() → Some.
    let dir = temp_dir("genome");
    std::fs::create_dir_all(&dir).unwrap();

    let axiom_path = dir.join("axiom.yaml");
    let genome_path = dir.join("genome.yaml"); // другой файл в той же директории
    std::fs::write(&axiom_path, AXIOM_YAML_TEMPLATE).unwrap();
    std::fs::write(&genome_path, "genome: test").unwrap();

    let watcher = ConfigWatcher::new(&axiom_path).unwrap();
    std::thread::sleep(Duration::from_millis(50));
    let _ = watcher.poll(); // drain startup events

    // Модифицируем genome.yaml — НЕ axiom.yaml
    std::fs::write(&genome_path, "genome: modified").unwrap();
    std::thread::sleep(Duration::from_millis(150));

    // ConfigWatcher фильтрует по имени файла — genome.yaml не должен триггерить
    let result = watcher.poll();
    assert!(
        result.is_none(),
        "genome.yaml не должен вызывать перезагрузку конфигурации"
    );

    std::fs::remove_dir_all(dir).ok();
}

#[test]
fn test_loaded_config_has_no_genome() {
    // LoadedAxiomConfig не содержит данных GENOME — по архитектуре
    // genome.yaml читается через axiom-genome, не через ConfigLoader::load_all
    let dir = temp_dir("nogenome");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("axiom.yaml");
    std::fs::write(&path, AXIOM_YAML_TEMPLATE).unwrap();

    let watcher = ConfigWatcher::new(&path).unwrap();
    std::thread::sleep(Duration::from_millis(50));
    let _ = watcher.poll();

    std::fs::write(&path, AXIOM_YAML_MODIFIED).unwrap();
    std::thread::sleep(Duration::from_millis(150));

    let cfg = watcher.poll().expect("expected new config");
    // LoadedAxiomConfig не имеет поля genome — это статически гарантировано типом
    // Просто проверяем что домены пустые (нет presets.domains_dir)
    assert!(cfg.domains.is_empty());
    assert!(cfg.heartbeat.is_none());

    std::fs::remove_dir_all(dir).ok();
}

