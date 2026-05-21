// ============================================================================
// MODULE  : Core Framework Error Invariants & Exception Matrix
// SUBSYSTEM: Zero-Allocation Diagnostic Architecture
// CRITERIA: Anti-Side-Channel Memory Safety Specification
// ============================================================================

/// Enterprise-grade enumerations for explicit runtime failure vectors.
/// Implemented without heap allocation to comply strictly with #![no_std].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkError {
    // --- Cryptographic Exception Vectors ---
    MpcThresholdNotMet,
    Gg18PolynomialDegreeMismatch,
    GaloisFieldInversionZeroDenominator,
    PaillierDecryptionBufferOverflow,
    HardwareEnclaveAttestationFailed,

    // --- Account Abstraction (ERC-4337) Exception Vectors ---
    UserOpNonceInvalid,
    BundlerGasLimitExceeded,
    PaymasterIncompleteCollateral,
    SimulationForbiddenOpcodeDetected,
    SignatureAggregationFailure,

    // --- Extension Layer Bridge Exception Vectors ---
    ExtLayerChannelInterrupted,
    SecureStorageReadViolation,
    SerializationPayloadMalformed,
}

impl FrameworkError {
    /// Returns a static string slice describing the anomaly for safe logging
    pub const fn to_static_str(&self) -> &'static str {
        match self {
            Self::MpcThresholdNotMet => "CRITICAL_ERROR: Required MPC signing threshold (T+1) participant matrix not satisfied.",
            Self::Gg18PolynomialDegreeMismatch => "CRYPTOGRAPHIC_ANOMALY: Shamir VSS polynomial evaluation indices mismatch.",
            Self::GaloisFieldInversionZeroDenominator => "MATHEMATICAL_VIOLATION: Attempted modular inverse calculation over a zero denominator.",
            Self::PaillierDecryptionBufferOverflow => "MEMORY_GUARD: Paillier homomorphic ciphertext length exceeds register boundaries.",
            Self::HardwareEnclaveAttestationFailed => "SECURITY_ALERT: WebAuthn Passkey hardware attestation signature validation rejected.",
            
            Self::UserOpNonceInvalid => "VALIDATION_FAILURE: Account abstraction execution nonce tracking is out of sequence.",
            Self::BundlerGasLimitExceeded => "MEMPOOL_REJECTION: Simulated aggregated gas requirements exceed max block limits.",
            Self::PaymasterIncompleteCollateral => "ECONOMIC_FAULT: Paymaster contract collateral deposit is insufficient for execution pre-fund.",
            Self::SimulationForbiddenOpcodeDetected => "SANDBOX_BREACH: UserOperation validation loop invoked restricted state-access opcodes.",
            Self::SignatureAggregationFailure => "AGGREGATION_FAULT: Aggregated signature matrix does not intersect over the target commitment root.",
            
            Self::ExtLayerChannelInterrupted => "BRIDGE_FAILURE: Inter-process memory pipeline with the ext-layer has been terminated.",
            Self::SecureStorageReadViolation => "ACCESS_DENIED: Unauthorized attempt to read from the hardware-backed volatile isolation vault.",
            Self::SerializationPayloadMalformed => "DESERIALIZATION_FAULT: Incoming execution payload byte stream violates schema formatting constraints.",
        }
    }
}
