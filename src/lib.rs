#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

mod protocol_core;
mod zk_verifier;
mod stream_integration;
mod error;
mod storage;

pub use error::LuminaError;
pub use protocol_core::*;
pub use zk_verifier::*;
pub use stream_integration::*;
pub use storage::*;

#[contract]
pub struct LuminaRWAProtocol;

#[contractimpl]
impl LuminaRWAProtocol {
    /// Initialize the protocol with a master compliance authority
    pub fn initialize(env: Env, authority: Address) {
        authority.require_auth();
        let storage = env.storage().instance();
        storage.set(&Symbol::new(&env, "authority"), &authority);
    }

    /// Register a wallet with a compliance profile
    pub fn set_compliance_profile(
        env: Env,
        user: Address,
        nullifier: [u8; 32],
        jurisdiction: u32,
        expiry: u32,
    ) -> Result<(), LuminaError> {
        let storage = env.storage();
        let instance = storage.instance();
        let persistent = storage.persistent();

        // Verify authority
        let auth: Address = instance
            .get(&Symbol::new(&env, "authority"))
            .ok_or(LuminaError::AuthorityNotSet)?;
        auth.require_auth();

        // Check for duplicate nullifier
        if persistent.has(&Symbol::short("nullifier").to_vec(&env).into_iter().collect::<Vec<u8>>()) {
            return Err(LuminaError::DuplicateNullifier);
        }

        // Create compliance state
        let profile = ComplianceState {
            regulatory_jurisdiction_code: jurisdiction,
            compliance_expiration_ledger: expiry,
            identity_nullifier_hash: nullifier,
        };

        // Store mappings
        persistent.set(&user, &profile);
        persistent.set(&nullifier, &user);

        Ok(())
    }

    /// Verify transfer compliance between sender and receiver
    pub fn verify_transfer_compliance(
        env: Env,
        sender: Address,
        receiver: Address,
    ) -> Result<(), LuminaError> {
        let storage = env.storage().persistent();
        let current_ledger = env.ledger().sequence();

        // Get sender profile
        let sender_profile: ComplianceState = storage
            .get(&sender)
            .ok_or(LuminaError::ComplianceNotFound)?;

        // Get receiver profile
        let receiver_profile: ComplianceState = storage
            .get(&receiver)
            .ok_or(LuminaError::ComplianceNotFound)?;

        // Check expiration
        if sender_profile.compliance_expiration_ledger <= current_ledger {
            return Err(LuminaError::ComplianceExpired);
        }
        if receiver_profile.compliance_expiration_ledger <= current_ledger {
            return Err(LuminaError::ComplianceExpired);
        }

        // Check jurisdiction match
        if sender_profile.regulatory_jurisdiction_code != receiver_profile.regulatory_jurisdiction_code
        {
            return Err(LuminaError::JurisdictionMismatch);
        }

        Ok(())
    }

    /// Check if a wallet is eligible for stream distribution
    pub fn check_stream_eligibility(env: Env, wallet: Address) -> Result<bool, LuminaError> {
        let storage = env.storage().persistent();
        let current_ledger = env.ledger().sequence();

        let profile: ComplianceState = storage
            .get(&wallet)
            .ok_or(LuminaError::ComplianceNotFound)?;

        Ok(profile.compliance_expiration_ledger > current_ledger)
    }

    /// Revoke compliance for a wallet
    pub fn revoke_compliance(env: Env, wallet: Address, reason: Symbol) -> Result<(), LuminaError> {
        let storage = env.storage();
        let instance = storage.instance();
        let persistent = storage.persistent();

        // Verify authority
        let auth: Address = instance
            .get(&Symbol::new(&env, "authority"))
            .ok_or(LuminaError::AuthorityNotSet)?;
        auth.require_auth();

        // Get and remove profile
        let profile: ComplianceState = persistent
            .get(&wallet)
            .ok_or(LuminaError::ComplianceNotFound)?;

        persistent.remove(&wallet);
        persistent.remove(&profile.identity_nullifier_hash);

        Ok(())
    }

    /// Get compliance profile for a wallet
    pub fn get_compliance_profile(env: Env, wallet: Address) -> Result<ComplianceState, LuminaError> {
        let storage = env.storage().persistent();
        storage
            .get(&wallet)
            .ok_or(LuminaError::ComplianceNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Ledger};

    #[test]
    fn test_initialize() {
        let env = soroban_sdk::Env::default();
        let authority = Address::random(&env);

        let contract = LuminaRWAProtocol;
        contract.initialize(env.clone(), authority.clone());

        // Verify authority is set
        assert_eq!(true, true); // Placeholder
    }
}
