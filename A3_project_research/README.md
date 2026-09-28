# Assignment 3 — Solana Project Research: **Phantom**

**Deliverables:** this research note · [`slides.pdf`](slides.pdf) (2 slides) · [`SPEAKER_SCRIPT.md`](SPEAKER_SCRIPT.md) (1-minute talk) · [Sources](#sources)

> **Why Phantom?** It's the default consumer wallet on Solana. Our own product, Mzia, is a voice-first Georgian layer that sits *on top of* wallets like Phantom, not a replacement. Understanding exactly what Phantom does on-chain vs off-chain shows where that layer adds value.

## 1. Problem & target users
Self-custody on Solana is hard: keys, seed phrases, token accounts, fees, and constant scam risk. Phantom packages this into a consumer app (browser extension + iOS/Android) so a regular user can hold, send, swap and use dApps without touching a CLI.
**Target users:** retail crypto users and traders (memecoins, NFTs, DeFi), and dApp developers who need a wallet their users already have. Phantom reports a peak of **~17 million monthly active users**. It has been multichain for a while (Solana, Ethereum, Base, Polygon, Bitcoin, …), but Solana is still its core. [1]

**Why Solana:** fees are a fraction of a cent and blocks come every ~400 ms, so frequent small actions (swaps, memecoin trades, NFT mints) are cheap and feel instant. SPL tokens and one shared set of standard programs mean a single wallet can support every token without per-token contract integrations.

## 2. Core workflow (end to end)
1. Install → create a wallet (seed phrase generated **on the device**) or sign in with an email/social login (embedded wallet).
2. Fund it: receive SOL/USDC to the public address, or buy with fiat through an on-ramp provider.
3. Use it: send tokens, swap in-app, or connect to a dApp ("Connect" → the dApp builds a transaction → Phantom simulates it and shows a preview → the user approves → Phantom signs and submits).
4. Check history and balances. Phantom reads these from its own indexer/RPC backend, not the raw chain.

## 3. User-facing features
Multi-chain balances, send/receive · in-app **swaps** (same-chain and cross-chain) · NFT gallery · staking · dApp connection (extension + mobile in-app browser) · **transaction simulation & scam warnings** · gasless swaps (network fee covered, taken from the swap output) · hardware-wallet (Ledger) support · Phantom Connect / embedded wallets for developers. [2][3][4]

## 4. Solana programs & standards used
Phantom doesn't deploy its own core program for normal wallet use. It **composes existing Solana programs** (all addresses verified as executable on mainnet on 2026-09-28):

| Program | Address | Used for |
|---|---|---|
| System Program | `11111111111111111111111111111111` | SOL transfers, account creation |
| SPL Token | `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA` | Fungible token transfers (`TransferChecked`) |
| Token-2022 | `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb` | Newer tokens with extensions (e.g. PYUSD) |
| Associated Token Account | `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL` | Creating the recipient's token account on first send |
| Compute Budget | `ComputeBudget111111111111111111111111111111` | Priority fees / compute limits |
| Jupiter Aggregator v6 | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` | Swap routing across DEXs (a common Solana swap provider) |
| Memo | `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr` | Optional notes on transfers |

**Standards:** Wallet Standard / `@solana/wallet-adapter` (how dApps detect and connect to Phantom), **Solana Pay** (QR payment requests), Metaplex token metadata (token names/icons, NFTs), versioned (v0) transactions with Address Lookup Tables.

## 5. Wallet & transaction flow
- **Keys:** a self-custodial Ed25519 keypair, encrypted locally on the device. Embedded (social-login) wallets are still self-custodial, but key management happens in Phantom's infrastructure instead of behind a seed phrase the user writes down.
- **Flow:** the dApp or Phantom builds the tx → **simulation** (balance changes, known-malicious checks) → human-readable preview → user approves → **Phantom signs locally** → it's sent through Phantom's RPC → confirmation.
- For embedded wallets, the SDK exposes only `signAndSendTransaction`. Every transaction goes through Phantom's simulation system first, which blocks malicious transactions and known-malicious origins. [3]
- **Example — basic transfer** (what Phantom does for "Send SOL"): our own [Devnet transfer](https://explorer.solana.com/tx/5Fib7N12VdxRPgt8us4BckHzABXQUjiWKgpWe8cc9adcz2syTFz5wESBvqRYdarSQTQ8wQysJeME3fUHq2ZCSUBe?cluster=devnet) from Assignment 1: one System Program `Transfer`, 1 signer, fee 5,000 lamports.
- **Example — aggregator swap** (the structure behind "Swap"): mainnet tx [`3vmLrVo6…FbRhKLj`](https://explorer.solana.com/tx/3vmLrVo6u4yLc3ZWoEh1sNZNbA5SnQgjhhefwKFDDXsXxbB121fij3boXcYRwY6RsbMs7pTuxtbs8WswvFbRhKLj) is a **v0 transaction** that uses **4 Address Lookup Tables**: 2× Compute Budget → Jupiter `RouteV2` → DEX `Swap` → SPL `TransferChecked`. It has **2 signers**, and a separate fee payer covers the network fee, which is the pattern behind "gasless" swaps. *(This is an illustrative Jupiter swap. We didn't confirm it came from Phantom specifically.)*

## 6. Tokens & on-chain assets
SOL (fees, rent, staking) · SPL and Token-2022 tokens: stablecoins (USDC, USDT, PYUSD, USDG), memecoins · NFTs and compressed NFTs · stake accounts (native staking with validators). Phantom itself has **no token**.

## 7. APIs, SDKs & infrastructure
Phantom Connect / Browser SDK (`@phantom/browser-sdk`) and React SDK for developers [3][5] · Wallet Standard / wallet-adapter · Solana RPC (Phantom runs its own RPC and indexing backend) · swap/bridge providers (Jupiter on Solana, plus cross-chain bridge providers) · fiat on-ramps · a transaction-simulation and threat-intelligence service · SimpleHash (acquired) for token and NFT data. [1]

## 8. Competitors & alternatives
**Solflare** (Solana-native, staking focus) · **Backpack** (wallet + exchange) · **Jupiter Mobile** (swap-first wallet) · **MetaMask** (added Solana) · **embedded wallets** like Privy (Stripe) and Dynamic, which dApps use to skip wallet installs entirely · exchange apps (Coinbase, Binance) for less crypto-native users.

## 9. Business model
The wallet is free. Revenue comes from **transaction flow**:
- a **0.85% fee** on most in-app swaps (shown before you confirm; routing through other providers) [2]
- bridge and cross-chain fees
- on-ramp referral revenue
- fees on newer financial products (perps, cash accounts)

Swap fees are the main lever. Users can avoid them by swapping directly on a DEX, which is a known criticism. [6]

## 10. Main technical risks
- **Phishing and drainers:** signatures the user doesn't understand. Simulation reduces this but doesn't remove it (for example, durable-nonce and delayed-execution tricks).
- **Supply chain:** a compromised extension update or dependency could reach millions of users.
- **Centralised off-chain dependencies:** the RPC, indexer and swap API are Phantom's servers. If they go down, the "wallet" looks broken even though the funds are safe on-chain.
- **Key loss:** seed phrases are still the #1 failure point for non-technical users. Embedded wallets add dependence on the provider's key infrastructure.
- **Network congestion:** priority-fee estimation and dropped transactions during memecoin spikes.

## 11. What could be improved
- **Localisation and accessibility:** the UX is English-first and screen-first. Users in emerging markets (e.g. Georgian speakers, older users) get no native-language or voice guidance. *This is exactly the gap Mzia fills, as a voice layer that works with existing wallets.*
- **Plain-language transaction previews:** explain *what will happen* ("you are giving this site permission to move all your USDC"), not just show balance diffs.
- **Trusted-recipient safety:** saved and verified contacts to fight address poisoning. (Our A2 `mzia_contacts` program is a prototype of this idea.)
- **Fee transparency:** show the 0.85% as an absolute amount next to a cheaper route.

## On-chain vs off-chain

| On-chain (verifiable on Solana) | Off-chain (Phantom's servers/app) |
|---|---|
| Balances, token accounts, stake accounts | Seed-phrase generation and encrypted key storage (on device) |
| Every transfer/swap instruction and its signature | Transaction simulation and scam detection |
| Swap routing (Jupiter/DEX programs), fee transfers | Portfolio UI, prices, activity history (indexer) |
| NFT ownership & metadata accounts | RPC endpoints, swap-quote API, on-ramps, notifications |

**Key insight:** Phantom's value is almost entirely **off-chain UX and security**. On-chain, it uses the same public programs any wallet can use. That's why new products can compete on experience (language, voice, safety) without needing better on-chain tech.

## Sources
1. Phantom blog — *From Crypto Product to Finance Platform* (≈17M peak MAU, SimpleHash acquisition): https://phantom.com/learn/blog/from-crypto-to-finance-platform
2. Phantom Help — *Swap tokens in Phantom* (0.85% fee, providers, cross-chain): https://help.phantom.com/hc/en-us/articles/5985106844435-Swap-tokens-in-Phantom
3. Phantom Developer Docs — *Sign and send transactions* (embedded wallets, simulation): https://docs.phantom.com/sdks/browser-sdk/sign-and-send-transaction
4. Phantom Help — *Gasless swaps on Solana*: https://help.phantom.com/hc/en-us/articles/41191481594643-Gasless-swaps-on-Solana-in-Phantom
5. Phantom Connect SDK (GitHub): https://github.com/phantom/phantom-connect-sdk
6. SolanaFloor — *Phantom attacked over swap fees*: https://solanafloor.com/news/phantom-attacked-swap-fees-reigniting-web3-revenue-stream-debate
7. Solana Docs — Programs, Tokens, Transactions: https://solana.com/docs · Jupiter docs: https://dev.jup.ag
8. Program addresses and the example transactions were verified directly via Solana RPC (`getAccountInfo`, `getTransaction`) on 2026-09-28.
