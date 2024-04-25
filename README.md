# Ord Sender - Make PSBTs for sending batches of Ordinals from external wallets

## Building

## Workflow

## Sample Configuration

### settings.toml

#### Singlesig taproot wallet on regtest
```toml
network = "regtest"
wallet_type = "SingleSigTaproot"

[id_resolver.OrdServer]
url = "http://localhost"
```

#### Multisig taproot wallet on mainnet
```toml
network = "bitcoin"
wallet_type = "MultiSigSegwit"

[id_resolver.OrdServer]
url = "http://localhost"
```