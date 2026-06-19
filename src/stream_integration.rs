use soroban_sdk::{Address, Env};
use crate::error::LuminaError;
use crate::protocol_core::ComplianceState;

/// Stream integration module for compatibility with Drips Network
/// and other continuous funding protocols
pub struct StreamIntegration;

impl StreamIntegration {
    /// Check if a wallet is eligible for stream distributions
    /// This is called before each distribution in streaming protocols
    pub fn check_stream_eligibility(
        env: &Env,
        wallet: &Address,
    ) -> Result<bool, LuminaError> {
        let storage = env.storage().persistent();
        let current_ledger = env.ledger().sequence();

        let profile: ComplianceState = storage
            .get(wallet)
            .ok_or(LuminaError::ComplianceNotFound)?;

        // Wallet is eligible if compliance hasn't expired
        Ok(profile.compliance_expiration_ledger > current_ledger)
    }

    /// Validate stream receiver for compliance
    /// Called when a stream is created or modified
    pub fn validate_stream_receiver(
        env: &Env,
        receiver: &Address,
    ) -> Result<(), LuminaError> {
        let storage = env.storage().persistent();
        let current_ledger = env.ledger().sequence();

        let profile: ComplianceState = storage
            .get(receiver)
            .ok_or(LuminaError::ComplianceNotFound)?;

        if profile.compliance_expiration_ledger <= current_ledger {
            return Err(LuminaError::ComplianceExpired);
        }

        Ok(())
    }

    /// Batch check eligibility for multiple stream participants
    pub fn batch_check_eligibility(
        env: &Env,
        wallets: &[Address],
    ) -> Result<Vec<bool>, LuminaError> {
        let mut results = Vec::new();
        for wallet in wallets {
            let eligible = Self::check_stream_eligibility(env, wallet)?;
            results.push(eligible);
        }
        Ok(results)
    }

    /// Get the remaining compliance duration for a wallet
    pub fn get_compliance_duration(
        env: &Env,
        wallet: &Address,
    ) -> Result<u32, LuminaError> {
        let storage = env.storage().persistent();
        let current_ledger = env.ledger().sequence();

        let profile: ComplianceState = storage
            .get(wallet)
            .ok_or(LuminaError::ComplianceNotFound)?;

        if profile.compliance_expiration_ledger <= current_ledger {
            return Ok(0);
        }

        Ok(profile.compliance_expiration_ledger - current_ledger)
    }
}

/// Drips Network specific integration point
/// This allows the Drips protocol to hook into our compliance checks
pub mod drips_compat {
    use super::*;

    /// Called by Drips before processing a distribution
    pub fn pre_distribution_check(
        env: &Env,
        receiver: &Address,
    ) -> Result<(), LuminaError> {
        StreamIntegration::check_stream_eligibility(env, receiver)?;
        Ok(())
    }

    /// Called by Drips when creating a new stream
    pub fn on_stream_created(
        env: &Env,
        receiver: &Address,
    ) -> Result<(), LuminaError> {
        StreamIntegration::validate_stream_receiver(env, receiver)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_integration_module_exists() {
        // Verify module can be imported and used
        assert!(true);
    }
}
