// ============================================================================
// FRAMEWORK: imToken Hybrid MPC-AA Enterprise Extension
// MODULE   : Root Library Definition & Architectural Exports
// LAYER    : Core Subsystem Compilation Guard
// ============================================================================

#![no_std]
#![deny(missing_debug_implementations)]
#![deny(unsafe_op_in_unsafe_fn)]

// Public export of discrete architectural layers
pub mod crypto;
pub mod aa_engine;
pub mod error;
pub mod ext_layer;

/// Global type aliases to enforce unified memory footprints across subsystems
pub type CryptographicNonce = [u8; 32];
pub type CompactAddress = [u8; 20];

/// Structural configuration flags parsed by the execution runtime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeConfiguration {
    pub strict_verification_mode: bool,
    pub max_simulation_gas_allowance: u64,
}

impl RuntimeConfiguration {
    /// Institutional default metrics matching mainnet simulation constraints
    pub const fn enterprise_default() -> Self {
        Self {
            strict_verification_mode: true,
            max_simulation_gas_allowance: 15_000_000,
        }
    }
}
