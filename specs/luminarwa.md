# LuminaRWA Protocol - Complete Specification

## Project Vision
An institutional-grade compliance engine built natively on Soroban that uses Zero-Knowledge proofs to secure the issuance, streaming, and lifecycle management of Real-World Assets (RWAs) while maintaining strict on-chain privacy.

## Requirements

### Functional Requirements

#### FR-1: ZK-Proof Identity Nullifiers
- System must validate compliance passes without exposing user credentials, geographic location, or transaction histories
- Nullifiers must be cryptographically secure [u8; 32] hashes
- Support for multiple compliance jurisdictions (regulatory_jurisdiction_code: u32)
- Expiration-based compliance validity with ledger sequence checking

#### FR-2: Compliance State Management
- Initialize protocol with master compliance authority
- Set compliance profiles mapping wallets to ZK-nullifiers
- Support jurisdiction codes and expiration ledgers
- Prevent duplicate nullifier mappings

#### FR-3: Transfer Access Controls
- Verify both sender and receiver hold valid, unexpired compliance profiles
- Enforce jurisdiction matching between sender and receiver
- Block transfers if either party has expired compliance
- Block cross-jurisdiction transfers

#### FR-4: Yield Stream Routing
- Integrate with streaming protocols (Drips Network compatible)
- Restrict automated distributions to verified, compliant wallets only
- Support continuous funding patterns with compliance checks

#### FR-5: Sanction Registry Integration
- Allow administrative blacklisting of wallets via cryptographic proofs
- Support immediate compliance revocation for regulatory breaches
- Maintain audit trail of compliance state changes

### Non-Functional Requirements

#### NFR-1: Performance
- Transfer compliance verification must execute in <100ms on-chain
- State lookups optimized for persistent storage queries
- Support for thousands of concurrent compliance profiles

#### NFR-2: Security
- All cryptographic operations use crypto-bigint for precision
- ZK-proof verification resistant to replay attacks
- Authority validation on all state-modifying operations
- No PII exposure on-chain

#### NFR-3: Reliability
- Contract must handle missing compliance profiles with clear error messages
- Graceful degradation if ledger sequence unavailable
- Atomic transactions for compliance profile updates

#### NFR-4: Compliance
- GDPR-compliant (no PII storage on-chain)
- KYC/AML compatible through nullifier abstraction
- OFAC sanction list compatible
- Audit-ready state management

## Design

### Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│             Institutional Issuer / KYC Provider         │
└────────────────────────────┬────────────────────────────┘
                             │ (Submits ZK Proof Payload)
                    ┌────────▼────────────┐
                    │   Soroban RPC       │
                    └────────┬────────────┘
                             │
        ┌────────────────────┴────────────────────┐
        │                                         │
  ┌─────▼──────┐                          ┌──────▼──────┐
  │ ZK Verifier │                          │  Compliance │
  │ (Math Ops)  │                          │ State Store │
  └─────┬──────┘                          └──────┬──────┘
        │                                        │
        └────────────────┬─────────────────────┘
                         │
        ┌────────────────▼──────────────────┐
        │  LuminaRWA Protocol Core           │
        │  - initialize()                    │
        │  - set_compliance_profile()        │
        │  - verify_transfer_compliance()    │
        │  - check_stream_eligibility()      │
        │  - revoke_compliance()             │
        └────────────────┬──────────────────┘
                         │
        ┌────────────────▼──────────────────┐
        │  Asset Transfers / Streams        │
        │  Verified & Compliant             │
        └───────────────────────────────────┘
```

### Core Components

#### 1. ComplianceState Struct
- `regulatory_jurisdiction_code: u32` - Jurisdiction identifier
- `compliance_expiration_ledger: u32` - Expiration block height
- `identity_nullifier_hash: [u8; 32]` - ZK nullifier

#### 2. Protocol Core Functions
- `initialize(env: Env, authority: Address)` - Setup protocol authority
- `set_compliance_profile(...)` - Register compliant wallet
- `verify_transfer_compliance(sender, receiver)` - Cross-contract validation
- `check_stream_eligibility(wallet)` - Stream registration check
- `revoke_compliance(wallet, reason)` - Administrative blacklist
- `get_compliance_profile(wallet)` - Query compliance status

#### 3. Storage Layers
- **Instance Storage**: Protocol authority (single, immutable)
- **Persistent Storage**: 
  - Wallet → ComplianceState mapping
  - Nullifier → Wallet reverse mapping
  - Revocation audit log

#### 4. Error Handling
- `ComplianceNotFound` - Missing compliance profile
- `ComplianceExpired` - Profile validity expired
- `JurisdictionMismatch` - Cross-border transfer blocked
- `DuplicateNullifier` - Nullifier already registered
- `UnauthorizedAuthority` - Only authority can modify state
- `NullifierRevoked` - Blacklisted wallet

### Data Flow

1. **Enrollment**:
   - KYC Provider submits ZK proof to authority
   - Authority calls `set_compliance_profile()` with nullifier
   - Wallet gains compliance status for jurisdiction + expiry

2. **Transfer**:
   - Token contract calls `verify_transfer_compliance(sender, receiver)`
   - Protocol checks both profiles exist and haven't expired
   - Protocol validates jurisdiction match
   - Transfer proceeds or reverts

3. **Streaming**:
   - Drips (or equivalent) calls `check_stream_eligibility(wallet)` before each distribution
   - Returns compliance status
   - Distribution happens only if eligible

4. **Revocation**:
   - Authority detects sanction or breach
   - Calls `revoke_compliance(wallet, reason)`
   - Nullifier immediately blacklisted
   - Subsequent transfers/streams blocked

## Implementation Tasks

### Phase 1: Core Contract (Prod-Ready)
1. Set up Cargo.toml with soroban-sdk and crypto-bigint dependencies
2. Implement ComplianceState serialization
3. Implement protocol initialization and authority management
4. Implement set_compliance_profile with duplicate prevention
5. Implement verify_transfer_compliance with full validation
6. Add comprehensive error handling
7. Write unit tests for all core functions
8. Add integration tests with mock token contracts

### Phase 2: Stream Integration
9. Implement check_stream_eligibility for Drips compatibility
10. Add stream-specific compliance checks
11. Test Drips protocol integration

### Phase 3: Revocation & Audit
12. Implement revoke_compliance function
13. Add audit logging for compliance changes
14. Implement compliance status query functions

### Phase 4: Production Hardening
15. Security audit for cryptographic operations
16. Performance benchmarking (<100ms requirement)
17. Gas optimization passes
18. Documentation and deployment guides

## Deployment Strategy

- **Testnet**: Deploy to Soroban testnet for integration testing
- **Staging**: Full integration test with mock Drips instance
- **Production**: Authority-gated deployment with multi-sig validation

---

**Status**: Ready for implementation  
**Version**: 1.0  
**Last Updated**: 2024
