
use std::mem;
use axiom_core::domain::DomainConfig;
fn main() { println!("DomainConfig size: {} bytes", mem::size_of::<DomainConfig>()); }

