// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2026 Chernov Denys
//
// startup — полная последовательность инициализации Axiom:
//   persist restore (если есть) → axiom.yaml → Genome → AxiomEngine
//   → AnchorSet → inject_anchor_tokens

use std::sync::Arc;

use anyhow::{Context, Result};
use tracing::{info, warn};

use axiom_config::{AnchorSet, ConfigLoader, LoadedAxiomConfig};
use axiom_persist::{AutoSaver, PersistenceConfig};
use axiom_runtime::AxiomEngine;

use crate::config::NodeConfig;

pub struct NodeState {
    pub engine: AxiomEngine,
    pub auto_saver: AutoSaver,
    pub anchor_set: Option<Arc<AnchorSet>>,
}

pub fn init(cfg: &NodeConfig) -> Result<NodeState> {
    std::fs::create_dir_all(&cfg.data_dir)
        .with_context(|| format!("failed to create data_dir {:?}", cfg.data_dir))?;

    // 1. Try to restore from disk; fall back to fresh engine from axiom.yaml
    let mut engine = match axiom_persist::load(&cfg.data_dir) {
        Ok(result) => {
            info!(
                "state restored: {} traces, {} tension traces",
                result.traces_imported, result.tension_imported
            );
            result.engine
        }
        Err(e) => {
            if matches!(e, axiom_persist::PersistError::NotFound(_)) {
                info!("no saved state — starting fresh");
            } else {
                warn!("state restore failed: {} — starting fresh", e);
            }
            let loaded = load_axiom_yaml(cfg)?;
            build_engine(&loaded)?
        }
    };

    // 2. Load AnchorSet, inject tokens, init ContextRecognizer subsystem refs
    let anchor_set = load_anchors(cfg)?;
    if let Some(ref anchors) = anchor_set {
        let injected = engine.inject_anchor_tokens(anchors);
        info!("injected {} anchor tokens", injected);
        engine.apply_anchor_set(anchors);
    }

    let auto_saver = AutoSaver::new(PersistenceConfig::new(1000));

    Ok(NodeState { engine, auto_saver, anchor_set })
}

fn load_axiom_yaml(cfg: &NodeConfig) -> Result<Option<LoadedAxiomConfig>> {
    if !cfg.axiom_yaml.exists() {
        warn!("axiom.yaml not found at {:?} — using defaults", cfg.axiom_yaml);
        return Ok(None);
    }

    let loaded = ConfigLoader::new()
        .load_all(&cfg.axiom_yaml)
        .with_context(|| format!("failed to load {:?}", cfg.axiom_yaml))?;

    info!("loaded axiom.yaml: {} domain presets", loaded.domains.len());
    Ok(Some(loaded))
}

fn build_engine(loaded: &Option<LoadedAxiomConfig>) -> Result<AxiomEngine> {
    let engine = match loaded {
        Some(cfg) => {
            let mut e = AxiomEngine::new();
            for (name, domain_cfg) in &cfg.domains {
                if domain_cfg.domain_id > 0 {
                    e.apply_domain_config(domain_cfg.domain_id, domain_cfg);
                    info!("applied domain config '{}'", name);
                }
            }
            e
        }
        None => {
            info!("using default AxiomEngine config");
            AxiomEngine::new()
        }
    };
    Ok(engine)
}

fn load_anchors(cfg: &NodeConfig) -> Result<Option<Arc<AnchorSet>>> {
    if !cfg.anchors_dir.exists() {
        warn!("anchors dir {:?} not found — FNV-1a fallback active", cfg.anchors_dir);
        return Ok(None);
    }

    match AnchorSet::load(&cfg.anchors_dir) {
        Ok(anchors) => {
            info!("loaded {} anchors from {:?}", anchors.axes.len(), cfg.anchors_dir);
            Ok(Some(Arc::new(anchors)))
        }
        Err(e) => {
            warn!("failed to load anchors: {} — FNV-1a fallback active", e);
            Ok(None)
        }
    }
}
