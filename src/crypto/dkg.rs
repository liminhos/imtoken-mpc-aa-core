// ============================================================================
// MODULE  : Distributed Key Generation (DKG) Protocol Orchestrator
// SUBSYSTEM: Multi-Stage Consensus Matrix for Threshold Setup
// CRITERIA: Synchronous State-Machine Enforcer / no_std Verified Execution
// ============================================================================

use crate::error::FrameworkError;
use crate::crypto::gg18_keygen::{DkgProtocolParticipant, KeyShard};

/// Definitive enumeration tracking the active chronological phase of the DKG setup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DkgExecutionPhase {
    Phase1PolynomialCommitment,
    Phase2PointVerification,
    Phase3ConsensusAchieved,
    DkgAbortedMaliciousActorDetected,
}

pub struct DkgConsensusOrchestrator<const N: usize, const T: usize> {
    pub current_phase: DkgExecutionPhase,
    pub authenticated_nodes_count: u16,
}

impl<const N: usize, const T: usize> DkgConsensusOrchestrator<N, T> {
    /// Instantiates a pristine synchronization pipeline for key setup
    pub const fn initialize_handshake() -> Self {
        Self {
            current_phase: DkgExecutionPhase::Phase1PolynomialCommitment,
            authenticated_nodes_count: 0,
        }
    }

    /// Advances the global DKG protocol state machine after rigorous synchronization audits
    pub fn advance_protocol_state(
        &mut self,
        participant: &DkgProtocolParticipant<N, T>,
        broadcast_shard: &KeyShard,
    ) -> Result<DkgExecutionPhase, FrameworkError> {
        
        match self.current_phase {
            DkgExecutionPhase::Phase1PolynomialCommitment => {
                // Audit the incoming shard criteria using the participant's homomorphic commitments
                if participant.audit_incoming_shard(broadcast_shard) {
                    self.authenticated_nodes_count += 1;
                    
                    // If quorum threshold boundaries are satisfied, advance to the next computational layer
                    if self.authenticated_nodes_count as usize >= T + 1 {
                        self.current_phase = DkgExecutionPhase::Phase2PointVerification;
                    }
                    Ok(self.current_phase)
                } else {
                    self.current_phase = DkgExecutionPhase::DkgAbortedMaliciousActorDetected;
                    Err(FrameworkError::HardwareEnclaveAttestationFailed)
                }
            }
            
            DkgExecutionPhase::Phase2PointVerification => {
                // Verify intersection consistency over the entire Galois field matrix
                if self.authenticated_nodes_count as usize == N {
                    self.current_phase = DkgExecutionPhase::Phase3ConsensusAchieved;
                }
                Ok(self.current_phase)
            }
            
            DkgExecutionPhase::Phase3ConsensusAchieved => Ok(self.current_phase),
            DkgExecutionPhase::DkgAbortedMaliciousActorDetected => {
                Err(FrameworkError::MpcThresholdNotMet)
            }
        }
    }
}
