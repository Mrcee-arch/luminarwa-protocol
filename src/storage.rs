use soroban_sdk::{Address, Env, Symbol};
use crate::protocol_core::ComplianceState;
use crate::error::LuminaError;

/// Storage layer abstraction for compliance state management
pub struct StorageManager;

impl StorageManager {
    // Storage keys
    pub fn key_authority() -> Symbol {
        Symbol::short("authority")
    }

    pub fn key_profile(wallet: &Address) -> Address {
        wallet.clone()
    }

    pub fn key_nullifier(nullifier: &[u8; 32]) -> [u8; 32] {
        *nullifier
    }

    pub fn key_audit_log() -> Symbol {
        Symbol::short("audit")
    }

    /// Store compliance profile for a wallet
    pub fn store_profile(
        env: &Env,
        wallet: &Address,
        profile: &ComplianceState,
    ) -> Result<(), LuminaError> {
        let storage = env.storage().persistent();
        storage.set(wallet, profile);
        Ok(())
    }

    /// Retrieve compliance profile for a wallet
    pub fn get_profile(
        env: &Env,
        wallet: &Address,
    ) -> Result<ComplianceState, LuminaError> {
        let storage = env.storage().persistent();
        storage
            .get(wallet)
            .ok_or(LuminaError::ComplianceNotFound)
    }

    /// Store reverse mapping from nullifier to wallet
    pub fn store_nullifier_mapping(
        env: &Env,
        nullifier: &[u8; 32],
        wallet: &Address,
    ) -> Result<(), LuminaError> {
        let storage = env.storage().persistent();
        storage.set(&nullifier, wallet);
        Ok(())
    }

    /// Retrieve wallet from nullifier
    pub fn get_wallet_by_nullifier(
        env: &Env,
        nullifier: &[u8; 32],
    ) -> Result<Address, LuminaError> {
        let storage = env.storage().persistent();
        storage
            .get(&nullifier)
            .ok_or(LuminaError::ComplianceNotFound)
    }

    /// Remove compliance profile
    pub fn remove_profile(
        env: &Env,
        wallet: &Address,
    ) -> Result<(), LuminaError> {
        let storage = env.storage().persistent();
        storage.remove(wallet);
        Ok(())
    }

    /// Remove nullifier mapping
    pub fn remove_nullifier_mapping(
        env: &Env,
        nullifier: &[u8; 32],
    ) -> Result<(), LuminaError> {
        let storage = env.storage().persistent();
        storage.remove(&nullifier);
        Ok(())
    }

    /// Check if wallet has compliance profile
    pub fn has_profile(
        env: &Env,
        wallet: &Address,
    ) -> bool {
        let storage = env.storage().persistent();
        storage.has(wallet)
    }

    /// Check if nullifier is already mapped
    pub fn has_nullifier(
        env: &Env,
        nullifier: &[u8; 32],
    ) -> bool {
        let storage = env.storage().persistent();
        storage.has(&nullifier)
    }

    /// Store authority in instance storage
    pub fn store_authority(
        env: &Env,
        authority: &Address,
    ) -> Result<(), LuminaError> {
        let storage = env.storage().instance();
        storage.set(&Self::key_authority(), authority);
        Ok(())
    }

    /// Get authority from instance storage
    pub fn get_authority(env: &Env) -> Result<Address, LuminaError> {
        let storage = env.storage().instance();
        storage
            .get(&Self::key_authority())
            .ok_or(LuminaError::AuthorityNotSet)
    }

    /// Log compliance state change (for audit purposes)
    pub fn log_compliance_change(
        env: &Env,
        wallet: &Address,
        action: &str,
        reason: &str,
    ) -> Result<(), LuminaError> {
        // In production, this would append to an audit log
        // For now, this is a placeholder
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_functions() {
        // Test that key functions are deterministic
        let key1 = StorageManager::key_authority();
        let key2 = StorageManager::key_authority();
        assert_eq!(key1, key2);
    }
}
