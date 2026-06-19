# LuminaRWA Protocol - Quick Start

## Build & Test

```bash
# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Run all tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Project Files Quick Reference

| File | Purpose |
|------|---------|
| `src/lib.rs` | Contract entry point, main functions |
| `src/protocol_core.rs` | ComplianceState, core data structures |
| `src/zk_verifier.rs` | ZK proof verification (framework) |
| `src/stream_integration.rs` | Drips Network compatibility |
| `src/error.rs` | Error types and codes |
| `src/storage.rs` | Storage abstraction layer |
| `specs/luminarwa.md` | Full specification |
| `specs/tasks.md` | Implementation tasks (1-15) |

## Core API

```rust
// Initialize
protocol.initialize(env, authority)?;

// Register compliant wallet
protocol.set_compliance_profile(env, user, nullifier, jurisdiction, expiry)?;

// Verify transfer
protocol.verify_transfer_compliance(env, sender, receiver)?;

// Check stream eligibility
protocol.check_stream_eligibility(env, wallet)?;

// Revoke compliance
protocol.revoke_compliance(env, wallet, reason)?;

// Get profile
protocol.get_compliance_profile(env, wallet)?;
```

## Error Codes Quick Reference

```
1 = ComplianceNotFound
2 = ComplianceExpired
3 = JurisdictionMismatch
4 = DuplicateNullifier
5 = UnauthorizedAuthority
6 = AuthorityNotSet
7 = NullifierRevoked
8 = InvalidNullifier
9 = InvalidJurisdiction
10 = InvalidExpiration
11 = ContractError
```

## Jurisdiction Codes

```
1 = US
2 = EU
3 = APAC
4 = UK
5 = CANADA
```

## Development Tasks (15 Total)

**Phase 1 (Core)**
- [ ] Task 1 - Setup & Dependencies
- [ ] Task 2 - ComplianceState struct
- [ ] Task 3 - Protocol initialization
- [ ] Task 4 - set_compliance_profile()
- [ ] Task 5 - verify_transfer_compliance()
- [ ] Task 6 - revoke_compliance()

**Phase 2 (Integration)**
- [ ] Task 7 - check_stream_eligibility()
- [ ] Task 8 - Error handling
- [ ] Task 9 - Unit tests (95%+ coverage)
- [ ] Task 10 - Integration tests

**Phase 3 (Hardening)**
- [ ] Task 11 - Performance benchmarking
- [ ] Task 12 - Security audit
- [ ] Task 13 - Production documentation
- [ ] Task 14 - Gas optimization
- [ ] Task 15 - Final build & verification

## Key Requirements

✅ Transfer verification: <100ms  
✅ Test coverage: 95%+  
✅ No PII on-chain  
✅ KYC/AML compliant  
✅ GDPR compliant  
✅ OFAC compatible  

## Git Workflow

```bash
# Create feature branch
git checkout -b feature/task-1-setup

# Work and commit
git add .
git commit -m "[Task 1] Setup project dependencies"

# Push
git push origin feature/task-1-setup
```

## Performance Targets

- `initialize()`: <5ms
- `set_compliance_profile()`: <50ms
- `verify_transfer_compliance()`: <100ms (critical)
- `check_stream_eligibility()`: <10ms
- `revoke_compliance()`: <30ms

## Testing Checklist

- [ ] Unit tests written
- [ ] Integration tests written
- [ ] Error cases tested
- [ ] Edge cases tested
- [ ] Coverage ≥95%
- [ ] All tests pass
- [ ] Gas usage measured

## Deployment Checklist

- [ ] Tests pass locally
- [ ] Coverage ≥95%
- [ ] Security audit complete
- [ ] Performance benchmarks met
- [ ] Documentation updated
- [ ] WASM builds successfully
- [ ] Testnet deployment verified

## Important Code Patterns

### Storage Access
```rust
let storage = env.storage().persistent();
storage.get(&key)?
storage.set(&key, &value);
storage.remove(&key);
storage.has(&key)
```

### Authority Verification
```rust
let auth: Address = env.storage().instance()
    .get(&Symbol::new(&env, "auth"))
    .ok_or(LuminaError::AuthorityNotSet)?;
auth.require_auth();
```

### Error Handling
```rust
let profile: ComplianceState = storage
    .get(&wallet)
    .ok_or(LuminaError::ComplianceNotFound)?;
```

## Common Commands

```bash
# Run specific test
cargo test test_initialize

# Run with output
cargo test -- --nocapture

# Single-threaded testing
cargo test -- --test-threads=1

# Check specific file
cargo check src/protocol_core.rs

# Generate documentation
cargo doc --open
```

## Resources

- Spec: `specs/luminarwa.md`
- Tasks: `specs/tasks.md`
- API: `README.md`
- Deployment: `DEPLOYMENT.md`
- Implementation: `IMPLEMENTATION_GUIDE.md`

## Contact & Support

- Issues: GitHub Issues
- Questions: Open discussion
- Security: security@drips.network

---

**Status**: Ready to Build  
**Total Tasks**: 15  
**Estimated Duration**: 2-3 weeks  

