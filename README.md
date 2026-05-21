# imToken Hybrid MPC-AA Enterprise Middleware Extension

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust: 1.75+](https://img.shields.io/badge/Rust-1.75%2B-blue.svg)](https://www.rust-lang.org/)
[![Architecture: no__std](https://img.shields.io/badge/Core-no__std-orange.svg)]()

An enterprise-grade, high-performance cryptographic middleware designed to seamlessly interface with the `token-core` architecture. This monorepo implements a secure, trustless, and seedless account restoration and transaction signing pipeline by uniting **Threshold Cryptography (MPC GG18)** with **Native Account Abstraction (ERC-4337)**.

---

## Technical Architecture & Cryptographic Primitives

The core execution environment is partitioned into three decoupled, deterministic sub-modules optimized for bare-metal deployment without standard library overhead (`#![no_std]`).


                          ┌────────────────────────┐
                          │     imToken Wallet     │
                          └───────────┬────────────┘
                                      │ (UserOp Entry)
                                      ▼
                          ┌────────────────────────┐
                          │  ext-bridge Component  │
                          └───────────┬────────────┘
                                      │
              ┌───────────────────────┴───────────────────────┐
              ▼                                               ▼
 ┌────────────────────────┐                      ┌────────────────────────┐
 │   core-crypto (MPC)    │                      │    aa-engine (4337)    │
 │ 🧠 Shamir Key Shards   │                      │ ⛽ Bundler Gas Sim     │
 │ 🔐 Homomorphic Masking │                      │ 📜 Paymaster Invariants│
 └────────────────────────┘                      └────────────────────────┘

 ### 1. Multi-Party Computation (MPC-GG18) Engine
Instead of deriving a single private key $sk$ which introduces a single point of failure, the key generation phase triggers a Distributed Key Generation (DKG) protocol over the elliptic curve $\text{secp256k1}$. The master secret is split into $N$ polynomial shards where any threshold $T+1$ participants can reconstruct or sign via Lagrange Interpolation at $x=0$:

$$f(x) = a_0 + a_1x + a_2x^2 + \dots + a_Tx^T \pmod p$$

Where the master secret is $f(0) = a_0$, and the prime modulus is the Mersenne Prime $p = 2^{31} - 1$. Partial signature shares are aggregated utilizing additive homomorphic masking vectors to execute non-interactive ECDSA signing handshakes:

$$s_i = k_i^{-1} (m + r \cdot sk_i) \pmod p$$

### 2. Native Account Abstraction (ERC-4337) Pipeline
Integrates custom transaction bundling mechanics directly at the wallet runtime level. It processes `PackedUserOperation` primitives using fixed-point memory constraints to mitigate flash-loan attack vectors during state execution loops.

---

## Directory Schema

* `core-crypto/` — Low-level Galois Field arithmetic, Paillier homomorphic encryption wrappers, and GG18 signing state machines.
* `aa-engine/` — Mempool bundler transaction simulations, gas estimation matrix, and secure paymaster validation logic.
* `ext-bridge/` — Inter-process communication layer interfacing memory spaces securely with `ext-layer` specifications.

---

## Verification & Compilation Profile

The mathematical frameworks contained within this repository are isolated from system-level calls. To compile the production target for embedded or WASM targets, execute:

```bash
cargo build --release --target wasm32-unknown-unknown
