use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum LuminaError {
    /// Compliance profile not found for wallet
    ComplianceNotFound = 1,

    /// Compliance profile has expired
    ComplianceExpired = 2,

    /// Sender and receiver jurisdictions do not match
    JurisdictionMismatch = 3,

    /// Nullifier is already mapped to another wallet
    DuplicateNullifier = 4,

    /// Only authority can perform this operation
    UnauthorizedAuthority = 5,

    /// Authority has not been set in the protocol
    AuthorityNotSet = 6,

    /// Wallet has been revoked from compliance
    NullifierRevoked = 7,

    /// Invalid nullifier hash
    InvalidNullifier = 8,

    /// Invalid jurisdiction code
    InvalidJurisdiction = 9,

    /// Invalid expiration ledger
    InvalidExpiration = 10,

    /// Generic contract error
    ContractError = 11,
}
