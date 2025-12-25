# Continuum AMM

**A Reactive, Policy-Driven Automated Market Maker**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Solana](https://img.shields.io/badge/built%20for-Solana-9945FF?logo=solana)](https://solana.com)

## Overview

Continuum is a next-generation Automated Market Maker (AMM) that treats liquidity as a dynamic, context-aware resource rather than a static pool. Unlike traditional AMMs where all provided liquidity is always active, Continuum selectively activates and deactivates liquidity provider (LP) positions in real time based on market conditions, observed metrics, and configurable policy rules.

This design delivers:
- Superior **capital efficiency** through contextual liquidity participation
- Built-in **risk mitigation** for LPs via automatic exposure control
- **Adaptive behavior** in response to volatility and market regimes
- Full **on-chain determinism** and auditability

> **Core Principle**: Liquidity should be conditional, reactive, and policy-governed.

## Key Features

- **Condition Logic**: Real-time gating of individual LP positions (ACTIVE / INACTIVE)
- **Liquidity Manager**: Aggregates only ACTIVE liquidity for swap execution
- **Continuum Core**: Policy-agnostic invariant engine for deterministic swaps
- **Observation Logic**: On-chain telemetry tracking price deltas, volume, and volatility signals
- **Policy Logic**: Threshold-based decision layer for adaptive protocol behavior

The system operates as a single deterministic pipeline with feedback: each swap generates observations → observations inform policies → policies reconfigure liquidity eligibility → next swap executes under updated conditions.

## Architecture

The protocol is implemented as a single on-chain program with tightly coupled but logically separated modules:

```
User → Swap Transaction
       ↓
Condition Logic (4.1) → Liquidity Manager (4.2) → Continuum Core (4.3)
       ↓                                          ↓
       ←────────────────────────────────────────────
                  Observation Logic (4.4)
                            ↓
                     Policy Logic (4.5)
                            ↓
               Future Liquidity Configuration
```

(Refer to the included architecture diagram for detailed data flows.)

## Comparison with Traditional AMMs

| Feature             | Traditional AMM          | Continuum                  |
|---------------------|--------------------------|----------------------------|
| Liquidity State     | Always active            | Conditional (ACTIVE/INACTIVE) |
| Risk Control        | External / manual        | Embedded / automatic       |
| Market Awareness    | None                     | Native telemetry           |
| Adaptivity          | Static parameters        | Reactive & policy-driven   |

## Status

This repository contains:
- The project whitepaper (`whitepaper.md`)
- Architecture diagram (`docs/architecture.png` or similar)
- Initial scaffolding / design specifications

The protocol is currently in the **design and specification phase**. Smart contract implementation is planned for the Solana blockchain using Anchor.

## Getting Started

(Placeholder – to be completed upon implementation)

```bash
# Clone the repository
git clone https://github.com/your-username/continuum-amm.git
cd continuum-amm

# Install dependencies (Anchor, Rust, etc.)
# Build and test
anchor test
```


## Documentation

- [`whitepaper.md`](whitepaper.md) – Full technical specification
- Architecture diagram – Visual representation of module interactions

## Security

All state transitions are explicit, bounded, and fully deterministic. No off-chain oracles or external inputs are required.

## Contributing

Contributions are welcome. Please open an issue first to discuss proposed changes.

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---

**Continuum is not just an AMM. It is a programmable liquidity control system.**