// ============================================================================
// MODULE  : External Extension Layer Bridge Registry
// SUBSYSTEM: imToken Core API Integration Layer
// LAYER    : Host IPC & Volatile Storage Gateway / no_std Isolation
// ============================================================================

pub mod bridge;
pub mod storage;

use crate::error::FrameworkError;

/// Binary layout specification for incoming and outgoing RPC frames
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionMessageFrame {
    pub call_identifier: u32,
    pub payload_checksum: [u8; 32],
    pub volatile_buffer_address: usize,
}

/// Strict trait enforcing standard secure-channel handshakes between 
/// the low-level MPC-AA engine and the imToken host application.
pub trait ExtensionLayerChannel {
    /// Dispatches an isolated memory frame to the extension host API
    fn transmit_secure_frame(&self, frame: &ExtensionMessageFrame) -> Result<(), FrameworkError>;

    /// Polls the host application for verified external state receipts
    fn receive_secure_frame(&self, call_id: u32) -> Result<ExtensionMessageFrame, FrameworkError>;
}
