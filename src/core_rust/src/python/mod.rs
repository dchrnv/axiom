// Axiom OS - Python Bindings v0.45.0
// PyO3-based bindings for Axiom OS core

#![cfg(feature = "python-bindings")]

use pyo3::prelude::*;

mod token;
mod intuition;
mod runtime;
mod signal_system;
pub mod modules;

use token::PyToken;
use intuition::{PyIntuitionEngine, PyIntuitionConfig};
use runtime::PyRuntime;
use signal_system::PySignalSystem;

/// Axiom OS Python Module (_core)
///
/// This is the low-level FFI module. Users should import `axiom` instead.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "Axiom Team")?;
    m.add("__license__", "AGPL-3.0-or-later")?;

    // Runtime (new in v0.45.0)
    m.add_class::<PyRuntime>()?;

    // Core types
    m.add_class::<PyToken>()?;

    // Intuition Engine
    m.add_class::<PyIntuitionEngine>()?;
    m.add_class::<PyIntuitionConfig>()?;

    // Signal System (new in v0.53.0)
    m.add_class::<PySignalSystem>()?;

    // Module Registry (new in v0.63.0)
    modules::register_module(m.py(), m)?;

    Ok(())
}
