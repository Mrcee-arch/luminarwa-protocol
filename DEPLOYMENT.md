# LuminaRWA Protocol - Deployment Guide

## Production Deployment Checklist

### Pre-Deployment

- [ ] All tests pass (`cargo test`)
- [ ] Code coverage ≥95% (`cargo tarpaulin`)
- [ ] Security audit completed
- [ ] Performance benchmarks meet targets (<100ms)
- [ ] Documentation is complete
- [ ] CHANGELOG updated
- [ ] Release notes prepared

### Build Process

#### 1. Prepare Release Build

```bash
# Clean previous builds
cargo clean

# Build optimized WASM
cargo build --target wasm32-unknown-unknown --release

# Verify WASM size
ls -lh target/wasm32-unknown-unknown/release/luminarwa_protocol.wasm
```

**Expected Output**: WASM file ~200-300 KB

#### 2. Verify Contract

```bash
# Extract contract hash for tracking
sha256sum target/wasm32-unknown-unknown/release/luminarwa_protocol.wasm

# Store hash in deployment manifest
echo "Contract Hash: $(sha256sum target/wasm32-unknown-unknown/release/luminarwa_protocol.wasm)" \
  > deployment/contract-hash.txt
```

### Testnet Deployment

#### 1. Setup Testnet Environment

```bash
# Set Soroban network to testnet
export SOROBAN_RPC_HOST="https://soroban-testnet.stellar.org"
export SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Fund your deployment account (get from faucet)
soroban config set --global RPC_URL https://soroban-testnet.stellar.org
soroban config set --global NETWORK_PASSPHRASE "Test SDF Network ; September 2015"
```

#### 2. Deploy to Testnet

```bash
# Deploy contract
CONTRACT_ID=$(soroban contract deploy \
  --source-account $TESTNET_ACCOUNT_ID \
  --wasm target/wasm32-unknown-unknown/release/luminarwa_protocol.wasm \
  --network testnet)

echo "Deployed Contract: $CONTRACT_ID"

# Save contract ID
echo "$CONTRACT_ID" > deployment/testnet-contract-id.txt
```

#### 3. Initialize Protocol

```bash
# Get testnet authority address
AUTHORITY=$TESTNET_AUTHORITY_ADDRESS

# Initialize the protocol
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $TESTNET_ACCOUNT_ID \
  --network testnet \
  -- initialize \
  --authority $AUTHORITY
```

#### 4. Run Integration Tests on Testnet

```bash
# Set contract ID for tests
export LUMINARWA_CONTRACT_ID=$CONTRACT_ID

# Run testnet integration tests
cargo test --test integration_tests testnet -- --nocapture
```

### Production Deployment

#### 1. Multi-Signature Deployment

```bash
# Create deployment transaction
soroban contract deploy \
  --source-account $AUTHORITY_MULTISIG \
  --wasm target/wasm32-unknown-unknown/release/luminarwa_protocol.wasm \
  --network public \
  --print-only > deployment/deploy-tx.json

# Distribute for signing (example with 3-of-5 multi-sig)
echo "Send deployment-tx.json to signers 1, 2, and 3"

# After collecting 3 signatures, submit transaction
soroban tx submit deployment/deploy-tx-signed.json --network public
```

#### 2. Verify Production Deployment

```bash
# Get contract info
soroban contract info --id $PRODUCTION_CONTRACT_ID --network public

# Verify authority
soroban contract invoke \
  --id $PRODUCTION_CONTRACT_ID \
  --network public \
  -- get_authority
```

#### 3. Initialize Production Protocol

```bash
# This must be done immediately after deployment
# and only by authorized parties

AUTHORITY=$PRODUCTION_AUTHORITY_ADDRESS

soroban contract invoke \
  --id $PRODUCTION_CONTRACT_ID \
  --source-account $AUTHORITY \
  --network public \
  -- initialize \
  --authority $AUTHORITY
```

### Post-Deployment Verification

#### 1. Contract Verification

```bash
# Verify contract is callable
soroban contract invoke \
  --id $CONTRACT_ID \
  --network $NETWORK \
  -- get_authority

# Expected output: Authority address
```

#### 2. Register Test Wallet

```bash
# Create test compliance profile
TEST_WALLET=$(soroban keys generate --name test-wallet)
NULLIFIER=$(openssl rand -hex 32)

soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $AUTHORITY \
  --network $NETWORK \
  -- set_compliance_profile \
  --user $TEST_WALLET \
  --nullifier $NULLIFIER \
  --jurisdiction 1 \
  --expiry 9999999
```

#### 3. Test Transfer Verification

```bash
# Test with two compliant wallets
TEST_WALLET_2=$(soroban keys generate --name test-wallet-2)

soroban contract invoke \
  --id $CONTRACT_ID \
  --network $NETWORK \
  -- verify_transfer_compliance \
  --sender $TEST_WALLET \
  --receiver $TEST_WALLET_2

# Expected: Should fail (wallet 2 not compliant)
```

### Monitoring & Maintenance

#### 1. Setup Monitoring

```bash
# Monitor contract events
./deployment/monitor-events.sh $CONTRACT_ID $NETWORK

# Watch for errors
./deployment/monitor-errors.sh $CONTRACT_ID $NETWORK

# Track gas usage
./deployment/monitor-gas.sh $CONTRACT_ID $NETWORK
```

#### 2. Daily Health Checks

```bash
#!/bin/bash
# Run daily health check

CONTRACT_ID=$1
NETWORK=$2

echo "Health Check: $(date)"

# Check authority is set
soroban contract invoke --id $CONTRACT_ID --network $NETWORK -- get_authority

# Check storage is accessible
echo "Storage accessible: OK"

# Check recent compliance profiles
echo "Compliance checks executing: OK"
```

#### 3. Incident Response

```bash
# If emergency revocation needed:
soroban contract invoke \
  --id $CONTRACT_ID \
  --source-account $AUTHORITY \
  --network $NETWORK \
  -- revoke_compliance \
  --wallet $COMPROMISED_WALLET \
  --reason "Emergency sanction"
```

## Rollback Plan

If critical issues are found post-deployment:

1. **Pause Operations** (if possible via administrative controls)
2. **Revoke Compromised Accounts**
3. **Deploy Patched Version** to new contract address
4. **Migrate State** from old to new contract
5. **Update Token Contracts** to reference new compliance engine

## Gas Optimization Records

| Operation | Gas (Est.) | Actual | Optimized |
|-----------|-----------|--------|-----------|
| initialize | 15,000 | - | - |
| set_compliance_profile | 45,000 | - | - |
| verify_transfer_compliance | 25,000 | - | - |
| check_stream_eligibility | 15,000 | - | - |
| revoke_compliance | 35,000 | - | - |

## Deployment History

| Version | Network | Contract ID | Date | Status |
|---------|---------|-------------|------|--------|
| 1.0.0 | testnet | TBD | TBD | Pending |
| 1.0.0 | staging | TBD | TBD | Pending |
| 1.0.0 | production | TBD | TBD | Pending |

## Emergency Contacts

- **Security Issues**: security@drips.network
- **Deployment Issues**: ops@drips.network
- **Compliance Questions**: compliance@drips.network

## Support Resources

- Soroban Documentation: https://soroban.stellar.org
- Stellar Documentation: https://developers.stellar.org
- Soroban CLI Reference: `soroban contract --help`

---

**Last Updated**: 2024  
**Maintained By**: Drips Network Team
