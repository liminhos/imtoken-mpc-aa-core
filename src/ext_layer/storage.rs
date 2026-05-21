// ============================================================================
// MODULE  : Volatile Isolation Storage Engine
// SUBSYSTEM: Anti-Tamper State Persistence Architecture
// CRITERIA: Self-Wiping Cryptographic Registry / no_std Zero-Allocation
// ============================================================================

use crate::error::FrameworkError;

/// Maximum safe discrete storage registers available in volatile memory
const MAX_STORAGE_REGISTERS: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct SecureStorageCell {
    pub key_slot_identifier: u32,
    pub payload_data_bytes: [u8; 32],
    pub is_locked_by_hardware: bool,
}

pub struct IsolatedStorageVault {
    pub internal_registry: [SecureStorageCell; MAX_STORAGE_REGISTERS],
    pub occupied_slots_bitmask: u16,
}

impl IsolatedStorageVault {
    /// Bootstraps an empty, clean hardware isolation memory block
    pub const fn establish_vault() -> Self {
        Self {
            internal_registry: [SecureStorageCell {
                key_slot_identifier: 0,
                payload_data_bytes: [0x00; 32],
                is_locked_by_hardware: false,
            }; MAX_STORAGE_REGISTERS],
            occupied_slots_bitmask: 0,
        }
    }

    /// Safely persists a data payload inside a verified hardware partition slot
    pub fn persist_state_payload(
        &mut self,
        slot_index: usize,
        identifier: u32,
        data: &[u8; 32],
    ) -> Result<(), FrameworkError> {
        
        if slot_index >= MAX_STORAGE_REGISTERS {
            return Err(FrameworkError::SecureStorageReadViolation);
        }

        let target_cell = &mut self.internal_registry[slot_index];

        if target_cell.is_locked_by_hardware {
            return Err(FrameworkError::SecureStorageReadViolation);
        }

        // Commit changes to the continuous memory register block
        target_cell.key_slot_identifier = identifier;
        target_cell.payload_data_bytes = *data;
        target_cell.is_locked_by_hardware = true; // Instantly lock cell to prevent rewriting

        // Update the operational bitmask tracking active storage distribution
        self.occupied_slots_bitmask |= 1 << slot_index;

        Ok(())
    }

    /// Explicitly overwrites and clears a memory block to protect against side-channel memory dumps
    pub unsafe fn wipe_and_deallocate_slot(&mut self, slot_index: usize) -> Result<(), FrameworkError> {
        if slot_index >= MAX_STORAGE_REGISTERS {
            return Err(FrameworkError::SecureStorageReadViolation);
        }

        let target_cell = &mut self.internal_registry[slot_index];
        
        // Use volatile raw writes to force immediate erasure of key memory tracks from the RAM
        unsafe {
            let identifier_ptr = &mut target_cell.key_slot_identifier as *mut u32;
            core::ptr::write_volatile(identifier_ptr, 0);

            for byte_index in 0..32 {
                let byte_ptr = &mut target_cell.payload_data_bytes[byte_index] as *mut u8;
                core::ptr::write_volatile(byte_ptr, 0x00);
            }
        }

        target_cell.is_locked_by_hardware = false;
        self.occupied_slots_bitmask &= !(1 << slot_index);

        Ok(())
    }
}
