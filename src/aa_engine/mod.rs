// ============================================================================
// MODULE  : Account Abstraction (ERC-4337) Core Registry
// SUBSYSTEM: Bundler Mempool Verification & Gas Optimization Pipeline
// LAYER    : Implementation Gatekeeper / Zero-Allocation Type Interface
// ============================================================================

pub mod user_op;
pub mod bundler;
pub mod paymaster;

use crate::error::FrameworkError;
use crate::aa_engine::user_op::PackedUserOperation;

/// Unified mathematical definition for big-endian 256-bit unsigned integers 
/// custom-built for embedded target optimization without external library overhead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U256 {
    pub limbs: [u64; 4],
}

impl U256 {
    /// Zero-initializer utility for gas accumulation metrics
    pub const fn zero() -> Self {
        Self { limbs: [0, 0, 0, 0] }
    }
}

/// Core interface establishing rigid semantic rules for standard ERC-4337 Paymasters.
/// Allows wallets to sponsor gas or accept payment in diverse ERC-20 token structures.
pub trait AccountAbstractionPaymaster {
    /// Validates the paymaster structural invariants and checks pre-fund deposits
    fn validate_paymaster_user_op(
        &self,
        user_op: &PackedUserOperation,
        required_prefund: U256,
    ) -> Result<[u8; 32], FrameworkError>;

    /// Post-execution handshake to reconcile gas slippage and return excess collateral
    fn post_operation_handshake(
        &self,
        context: &[u8; 32],
        actual_gas_cost: U256,
    ) -> Result<(), FrameworkError>;
}
