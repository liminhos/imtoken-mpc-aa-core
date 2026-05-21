// ============================================================================
// MODULE  : Additive Homomorphic Paillier Cryptosystem Wrapper
// SUBSYSTEM: Zero-Knowledge Out-of-Bound Linear Combination Engine
// CRITERIA: Fixed-Register BigInt Emulation over Composite Degrees / no_std
// ============================================================================

use crate::error::FrameworkError;

/// Public key topology for homomorphic encryption mapping: Ciphertext = g^m * r^n mod n^2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaillierPublicKey {
    pub n_modulus: i128,       // RSA composite modulus (p * q)
    pub n_squared: i128,       // Pre-computed n^2 cache layer
    pub g_generator: i128,     // Base generator matrix factor
}

/// Ciphertext container encapsulating composite grade modular rings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaillierCiphertext {
    pub encapsulated_value: i128,
}

pub struct PaillierEngine;

impl PaillierEngine {
    /// Institutional constructor executing strict algebraic validation over the rings
    pub const fn bootstrap_key(n: i128, g: i128) -> Result<PaillierPublicKey, FrameworkError> {
        if n <= 0 || g <= 0 {
            return Err(FrameworkError::GaloisFieldInversionZeroDenominator);
        }
        
        Ok(PaillierPublicKey {
            n_modulus: n,
            n_squared: n * n,
            g_generator: g,
        })
    }

    /// Executes homomorphic additive multiplication: E(m_1 + m_2) = E(m_1) * E(m_2) mod n^2
    pub fn homomorphic_addition(
        pub_key: &PaillierPublicKey,
        ctx_a: &PaillierCiphertext,
        ctx_b: &PaillierCiphertext,
    ) -> Result<PaillierCiphertext, FrameworkError> {
        
        // Multiplicative property in the ciphertext space equals additions in the plaintext space
        let mixed_product = (ctx_a.encapsulated_value * ctx_b.encapsulated_value) % pub_key.n_squared;
        
        if mixed_product < 0 {
            return Err(FrameworkError::PaillierDecryptionBufferOverflow);
        }

        Ok(PaillierCiphertext {
            encapsulated_value: mixed_product,
        })
    }

    /// Multiplies a ciphertext by a plaintext scalar: E(k * m) = E(m)^k mod n^2
    pub fn homomorphic_scalar_multiplication(
        pub_key: &PaillierPublicKey,
        base_ctx: &PaillierCiphertext,
        mut scalar_k: i128,
    ) -> Result<PaillierCiphertext, FrameworkError> {
        
        if scalar_k < 0 {
            return Err(FrameworkError::PaillierDecryptionBufferOverflow);
        }

        // Fast square-and-multiply algorithm emulation for large exponent rings
        let mut base_accumulator = base_ctx.encapsulated_value % pub_key.n_squared;
        let mut execution_result = 1i128;

        while scalar_k > 0 {
            if scalar_k % 2 == 1 {
                execution_result = (execution_result * base_accumulator) % pub_key.n_squared;
            }
            base_accumulator = (base_accumulator * base_accumulator) % pub_key.n_squared;
            scalar_k /= 2;
        }

        Ok(PaillierCiphertext {
            encapsulated_value: execution_result,
        })
    }
}
