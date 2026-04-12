# AXIOM Configuration System Specification V1.0

**Version:** 1.1
**Status:** Active
**Scope:** Runtime configuration and semantic schema definition for Axiom.

---

# 1. Purpose

The configuration system defines how Axiom loads, validates, and applies configuration and schema data.

The system has two responsibilities:

1. Configure how the system runs.
2. Define the semantic structure of the Axiom environment.

To support these responsibilities, configuration is divided into two independent layers:

- **Runtime Configuration**
- **Schema Configuration**

This separation ensures that infrastructure settings remain stable while the semantic model of the system can evolve.

---

# 2. Design Principles

The configuration system follows several core principles.

**1. Strong Typing**

All configuration is represented as strongly typed structures in code.
Configuration files are only serialized representations of these structures.

---

**2. Declarative Configuration**

Configuration files describe system state but do not contain executable logic.

Allowed:

- structural definitions
- parameters
- constraints

Forbidden:

- conditional logic
- algorithms
- procedural behavior

---

**3. Separation of Concerns**

Runtime configuration and semantic schema must remain separate.

Runtime config describes **how the system operates**.
Schema config describes **the structure of the semantic environment**.

---

**4. Minimal Infrastructure**

Configuration loading must remain simple and predictable.

The configuration system must not evolve into a secondary runtime.

---

**5. Deterministic Initialization**

All configuration must be loaded and validated before runtime initialization begins.

Runtime behavior must never depend on partially loaded configuration.

---

# 3. Configuration Layers

## 3.1 Runtime Configuration

Runtime configuration defines operational parameters of the system.

Typical responsibilities include:

- runtime environment
- logging
- resource limits
- storage
- network configuration
- performance parameters

Runtime configuration changes rarely and is expected to remain stable.

Example:

```yaml
runtime:
  threads: 4
  max_tokens: 100000
  cache_size: 512mb
```

---

## 3.2 Schema Configuration

Schema configuration defines the semantic structure of the Axiom system.

This includes:

- domains
- token types
- connection rules
- grid structure
- layer definitions
- field parameters

Schema configuration may evolve over time as the semantic model changes.

Example:

```yaml
token_types:
  - concept
  - relation
  - context

connection_rules:
  max_degree: 16
  symmetry: true
```

Schema files describe **semantic physics of the system**, not runtime behavior.

---

# 4. Configuration File Format

Configuration files use **YAML** as the primary format.

Reasons:

- human readability
- support for comments
- widespread ecosystem support

JSON is implicitly supported because YAML is a superset of JSON.

---

# 5. Directory Structure

The configuration system follows a strict directory structure.

```
config/
    runtime/
        runtime.yaml
        logging.yaml

schema/
        grid.yaml
        domain.yaml
        token.yaml
        connection.yaml
```

Runtime and schema configuration must never be mixed within the same file.

---

# 6. Root Configuration File

Axiom uses a root configuration file that references module configuration files.

Example:

```yaml
version: 1

runtime:
  runtime: config/runtime/runtime.yaml
  logging: config/runtime/logging.yaml

schema:
  grid: config/schema/grid.yaml
  domain: config/schema/domain.yaml
  token: config/schema/token.yaml
  connection: config/schema/connection.yaml
```

The root configuration acts as the entry point for the configuration loader.

---

# 7. Configuration Loader

Axiom uses a single configuration loader responsible for:

1. reading the root configuration
2. resolving referenced files
3. parsing YAML
4. constructing typed structures
5. validating all configuration
6. returning a fully initialized configuration object

The loader performs no runtime logic beyond initialization.

Loader workflow:

```
read root config
resolve file paths
parse yaml
build structs
validate
return configuration
```

---

# 8. Validation

All configuration structures must implement validation.

Validation is defined through a lightweight validation interface.

Example concept:

```
trait Validate {
    fn validate(&self) -> Result<()>;
}
```

Validation ensures:

- type correctness
- required fields
- valid ranges
- structural consistency
- schema compatibility

Validation must run before the system enters runtime execution.

---

# 9. Versioning

The root configuration file must contain a version field.

Example:

```yaml
version: 1
```

This allows future schema migrations and compatibility checks.

---

# 10. Ownership Rules

Each system module owns its configuration.

Example:

```
core/domain/config.rs
core/token/config.rs
core/connection/config.rs
```

Each module defines:

- configuration struct
- default values
- validation rules

Modules must not modify configuration belonging to other modules.

---

# 11. Runtime Interaction

Configuration is read-only after initialization.

Runtime components may read configuration but must never mutate it.

Dynamic runtime behavior must be implemented in runtime systems, not in configuration.

---

# 12. Hot Reload

The configuration system supports live reload of configuration without process restart.

## 12.1 Mechanism

A `ConfigWatcher` subscribes to filesystem events on the configuration directory via the platform notification API (inotify on Linux, FSEvents on macOS, ReadDirectoryChanges on Windows).

When the root configuration file changes, `ConfigWatcher::poll()` returns a freshly loaded `LoadedAxiomConfig`.

The poll operation is non-blocking. Multiple file change events between poll calls are collapsed into a single reload.

## 12.2 Scope

Hot reload applies only to configuration that can be safely mutated at runtime.

**Reloadable:**
- `TickSchedule` — periodic task intervals applied to a running Engine instance

**Not reloadable (requires restart):**
- tick frequency (`tick_hz`)
- output verbosity
- data directory path
- any parameter that affects initialization state

## 12.3 Invariants

- The watcher never blocks the runtime loop.
- A failed reload is silently ignored — the previous configuration remains active.
- GENOME configuration is explicitly excluded from hot reload scope and must never be reloaded at runtime.

---

# 13. Component Configuration Loading

## 13.1 HeartbeatConfig

`HeartbeatConfig` defines periodic activation parameters for background processes (Heartbeat V2.0).

It is loaded as part of `load_all` when `presets.heartbeat_file` is specified in the root configuration.

```
presets:
  heartbeat_file: "presets/heartbeat.yaml"
```

If the path is absent or the file does not exist, `LoadedAxiomConfig.heartbeat` is `None`. This is not an error — the runtime uses its own default.

`HeartbeatConfig` fields:

| Field | Type | Description |
|-------|------|-------------|
| `interval` | `u32` | Events between pulses (must be > 0) |
| `batch_size` | `usize` | Tokens added to frontier per pulse |
| `connection_batch_size` | `usize` | Connections added to frontier per pulse |
| `enable_decay` | `bool` | Token decay activation |
| `enable_gravity` | `bool` | Gravitational update activation |
| `enable_spatial_collision` | `bool` | Spatial collision checks |
| `enable_connection_maintenance` | `bool` | Connection maintenance |
| `enable_thermodynamics` | `bool` | Thermodynamic processes |
| `attach_pulse_id` | `bool` | Attach pulse_id to generated events |
| `enable_shell_reconciliation` | `bool` | Shell V3.0 reconciliation |

Built-in presets available in code: `weak()`, `medium()`, `powerful()`, `disabled()`.

## 13.2 Presets Directory Layout

```
config/
  axiom.yaml
  presets/
    domains/        ← DomainConfig YAML files (one per domain)
    tokens/         ← TokenPreset YAML files
    connection/     ← ConnectionPreset YAML files
    spatial/        ← SpatialConfig YAML files
    heartbeat.yaml  ← HeartbeatConfig (optional, path in axiom.yaml)
  schema/
    semantic_contributions.yaml
    domain.yaml / token.yaml / connection.yaml / grid.yaml / upo.yaml
  runtime/
    runtime.yaml / runtime_schema.yaml
```

---

# 14. Future Extensions

The configuration system is designed to support future capabilities without architectural changes.

Possible future features include:

- schema version migration
- configuration validation CLI (`axiom-cli --dump-schema`, D-07)
- JSON Schema validation via `schemars` + `jsonschema` (D-07, requires internet)
- environment profiles
- configuration diff and inspection

These features must operate on the existing configuration structures without changing the core architecture.

---

# 15. Non-Goals

The configuration system explicitly does not support:

- runtime scripting
- configuration plugins
- executable configuration logic
- dynamic configuration mutation

These capabilities introduce instability and are intentionally excluded.

---

# 16. Summary

The Axiom configuration system provides a stable foundation based on several key ideas:

- strong typing
- strict separation between runtime and schema
- declarative configuration
- minimal infrastructure
- deterministic initialization

This architecture allows Axiom to evolve its semantic model while maintaining a stable runtime foundation.

The configuration system is intentionally simple so that the complexity of the system remains within the semantic model and runtime architecture rather than the configuration infrastructure.
