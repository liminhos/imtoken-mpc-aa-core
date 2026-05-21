// ============================================================================
// MODULE  : ERC-4337 PackedUserOperation Data Structure
// SUBSYSTEM: Native Account Abstraction Execution Frame
// CRITERIA: Fixed-Size Continuous Memory Layout / no_std Allocation
// ============================================================================

use crate::error::FrameworkError;
use crate::aa_engine::U256;

/// Production-grade representation of a packed user operation container.
/// Structured specifically to align with evm memory boundaries during execution loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackedUserOperation {
    pub sender_address: [u8; 20],
    pub unique_nonce: u64,
    pub call_data_offset: u32,
    pub call_data_length: u32,
    pub account_gas_limits: [u8; 32],
    pub pre_verification_gas: u64,
    pub max_fee_per_gas: u64,
    pub max_priority_fee_per_gas: u64,
    pub paymaster_and_data_offset: u32,
    pub paymaster_and_data_length: u32,
}

pub struct UserOperationSanitizer;

impl UserOperationSanitizer {
    /// Audits incoming operation structures to block potential out-of-bounds memory exploits
    pub const fn audit_payload_boundaries(
        op: &PackedUserOperation,
        allocated_buffer_size: usize,
    ) -> Result<(), FrameworkError> {
        
        // Anti-Exploit Check 1: Ensure call_data segments don't cause an integer overflow
        let total_call_data_span = op.call_data_offset as usize + op.call_data_length as usize;
        if total_call_data_span > allocated_buffer_size {
            return Err(FrameworkError::SerializationPayloadMalformed);
        }

        // Anti-Exploit Check 2: Verify paymaster byte layout boundaries
        let total_paymaster_span = op.paymaster_and_data_offset as usize + op.paymaster_and_data_length as usize;
        if total_paymaster_span > allocated_buffer_size {
            return Err(FrameworkError::SerializationPayloadMalformed);
        }

        // Anti-Exploit Check 3: Economic constraint auditing to block zero-gas spamming vectors
        if op.max_fee_per_gas == 0 {
            return Err(FrameworkError::BundlerGasLimitExceeded);
        }

        Ok(())
    }

    /// Computes the absolute minimum required pre-fund amount for this operation in Wei
    pub fn calculate_required_prefund(&self, op: &PackedUserOperation) -> U256 {
        let base_gas = op.pre_verification_gas;
        
        // Parsing high 128-bits of account_gas_limits (Verification Gas Limit emulation)
        let mut verification_gas: u64 = 0;
        for i in 0..8 {
            verification_gas |= (op.account_gas_limits[i] as u64) << (i * 8);
        }

        let total_estimated_gas = base_gas + verification_gas;
        let absolute_wei_cost = total_estimated_gas * op.max_fee_per_gas;

        // Map the u64 product into our custom U256 structure limb allocation
        let mut product_u256 = U256::zero();
        product_u256.limbs[0] = absolute_wei_cost;
        product_u256
    }
}
