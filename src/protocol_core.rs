use soroban_sdk::contracttype;

/// Represents the compliance state of a wallet
/// This struct encapsulates all compliance-related information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComplianceState {
    /// Regulatory jurisdiction code (e.g., US=1, EU=2, APAC=3)
    pub regulatory_jurisdiction_code: u32,

    /// Ledger sequence number at which this compliance expires
    pub compliance_expiration_ledger: u32,

    /// Zero-knowledge nullifier hash representing the identity
    /// This 32-byte hash obscures the actual identity while enabling verification
    pub identity_nullifier_hash: [u8; 32],
}

impl ComplianceState {
    /// Create a new compliance state
    pub fn new(
        jurisdiction: u32,
        expiry: u32,
        nullifier: [u8; 32],
    ) -> Self {
        ComplianceState {
            regulatory_jurisdiction_code: jurisdiction,
            compliance_expiration_ledger: expiry,
            identity_nullifier_hash: nullifier,
        }
    }

    /// Validate compliance state parameters
    pub fn is_valid(&self, current_ledger: u32) -> bool {
        self.compliance_expiration_ledger > current_ledger
            && self.regulatory_jurisdiction_code > 0
    }
}

/// Jurisdiction codes for compliance zones
pub mod jurisdictions {
    pub const US: u32 = 1;
    pub const EU: u32 = 2;
    pub const APAC: u32 = 3;
    pub const UK: u32 = 4;
    pub const CANADA: u32 = 5;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_state_creation() {
        let state = ComplianceState::new(1, 1000, [0u8; 32]);
        assert_eq!(state.regulatory_jurisdiction_code, 1);
        assert_eq!(state.compliance_expiration_ledger, 1000);
    }

    #[test]
    fn test_is_valid_not_expired() {
        let state = ComplianceState::new(1, 1000, [0u8; 32]);
        assert!(state.is_valid(500));
    }

    #[test]
    fn test_is_valid_expired() {
        let state = ComplianceState::new(1, 500, [0u8; 32]);
        assert!(!state.is_valid(1000));
    }
}
