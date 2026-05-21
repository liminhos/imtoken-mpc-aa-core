// ============================================================================
// MODULE  : Cryptographic Engine Module Registry
// SUBSYSTEM: Multi-Party Computation & Finite Field Topologies
// LAYER    : Core Primitive Aggregator
// ============================================================================

// Declaring internal sub-modules for structural isolation
pub mod gg18_keygen;
pub mod gg18_sign;
pub mod paillier;
pub mod dkg;

use crate::error::FrameworkError;

/// Core mathematical traits enforcing standard execution interfaces 
/// across diverse multi-party threshold schemes.
pub trait ThresholdScheme {
    type SecretShare;
    type SignatureOutput;
    type PublicCommitment;

    /// Evaluates if the internal state has achieved cryptographic quorum verification
    fn is_quorum_validated(&self, active_participants: u16) -> bool;

    /// Safe execution pipeline executing threshold calculations inside an isolated matrix
    fn execute_secure_pipeline(
        &self,
        share: &Self::SecretShare,
        payload: &[u8; 32],
    ) -> Result<Self::SignatureOutput, FrameworkError>;
}

/// Cryptographic context container maintaining public validation coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicIdentityContext {
    pub aggregated_public_key: [u8; 65], // Uncompressed Secp256k1 public key matrix
    pub threshold_limit: u16,
    pub total_parties: u16,
}

impl PublicIdentityContext {
    /// Institutional constructor enforcing rigorous parameter sanitization
    pub const fn new(public_key: [u8; 65], threshold: u16, total: u16) -> Result<Self, FrameworkError> {
        if threshold == 0 || threshold >= total {
            return Err(FrameworkError::Gg18PolynomialDegreeMismatch);
        }
        Ok(Self {
            aggregated_public_key: public_key,
            threshold_limit: threshold,
            total_parties: total,
        })
    }
}
