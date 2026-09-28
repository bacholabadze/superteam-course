# Assignment 4 — Solana Project Research: **Jupiter**

**Deliverables:** this research note · [`slides.pdf`](slides.pdf) (2 slides) · [`SPEAKER_SCRIPT.md`](SPEAKER_SCRIPT.md) (1-minute talk) · [Sources](#sources)

> **Why Jupiter?** In Assignment 3 we studied Phantom, a *wallet* that runs almost entirely on public programs. Jupiter is the opposite end of the stack: **on-chain DeFi infrastructure** with its own programs (swap router, limit orders, DCA, perpetuals), which Phantom and most other Solana wallets route through. For Mzia, Jupiter's Swap and Price APIs are the realistic way to support "convert to USDT" by voice.

## 1. Problem & target users
Solana liquidity is spread across dozens of DEXs and AMMs (Raydium, Orca, Meteora, pump-style bonding curves, …). Getting the best price by hand means checking every pool, splitting orders and handling slippage. **Jupiter is a liquidity aggregator**: one quote, the best route across all venues, one transaction.
**Target users:**
- traders, through the jup.ag web app
- **developers and wallets**, through the APIs. Jupiter's own docs say its Price API is "used by Phantom, Solflare, and most Solana wallets/apps". [1]

Over time Jupiter has grown into a DeFi "super-app": limit orders, DCA, perpetuals, lending, a stablecoin (JupUSD) and prediction markets. [1]

**Why Solana:** routing a single swap through several pools needs many accounts and instructions in one atomic transaction. Solana makes that cheap (sub-cent fees) and fast (~400 ms blocks), and **Address Lookup Tables** in v0 transactions let one transaction reference dozens of accounts.

## 2. Core workflow (end to end)
1. The user picks input and output tokens and an amount (in jup.ag or any app using the API).
2. `GET /swap/v2/order` returns a quote. **Routing engines compete**: Metis (on-chain routing across DEXs), JupiterZ (RFQ market makers), plus third-party routers. The best one wins. [2]
3. The response includes an **assembled v0 transaction**. The user signs it in their wallet.
4. `POST /swap/v2/execute` has Jupiter land the transaction (priority fees, retries, MEV protection) and poll for confirmation. [2]
5. On-chain, the Jupiter v6 program executes the route atomically: the whole swap either succeeds or nothing happens.

For developers who need control, `GET /swap/v2/build` returns **raw instructions** instead (Metis routing only, **no Jupiter swap fee**). They then build, sign and send the transaction themselves. [3]

## 3. User-facing features
Best-price swaps · **gasless swaps** (Jupiter or a market maker pays the network fee, recouped from the swap) [4] · limit orders (single, OCO, OTOCO) · DCA / recurring buys · **perpetuals** (up to high leverage on SOL, ETH, BTC) backed by the **JLP** liquidity pool · Jupiter Lend (earn / borrow / flashloans) · token verification & "organic score" · prices for any SPL token · an embeddable swap widget (Jupiter Plugin). [1]

## 4. Solana programs, protocols & standards
Unlike Phantom, **Jupiter runs its own on-chain programs** (all verified as executable on mainnet via RPC, 2026-09-28):

| Program / asset | Address | Role |
|---|---|---|
| **Jupiter Aggregator v6** | `JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4` | Swap router: `RouteV2`, `SharedAccountsRoute`; CPIs into each DEX. Upgradeable; ~2.9 MB program |
| **Perpetuals** | `PERPHjGBqRHArX4DySjwM6UJHiR3sWAatqfdBS2qQJu` | Perps positions + the JLP pool (Pool, Custody, Position, PositionRequest accounts) [5] |
| **DCA** | `DCA265Vj8a9CEuX1eb1LWRnDT7uK6q1xMipnNyatn23M` | Recurring buys; a keeper fills them through the v6 router |
| **Limit Order v2** | `j1o2qRpjcyUwEvwtcfhEQefh773ZgjxcVRry7LDqg5X` | Limit orders (`InitializeOrder`, …) |
| Limit Order v1 (legacy) | `jupoNjAxXgZ4rjzxzPMP4oxduvQsQtZzyknqvzYNrNu` | Older limit-order program |
| Oracle used by Perps | `DoVEsk76QybCEHQGzkvYPWLQu9gzNoZZZt3TPiL597e` | Receives signed price updates (Pyth Lazer + `Ed25519SigVerify`) |
| **JUP token** (SPL mint) | `JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN` | Governance token; supply ≈ **6.86B** (6 decimals, read on-chain) |
| **JLP token** (SPL mint) | `27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4` | LP share of the perps pool; supply ≈ **194.0M** (read on-chain) |
| Gasless fee payer | `gasTzr94Pmp4Gf8vknQnqxeYxdgwFjbgdJa4msYRpnB` | Jupiter-sponsored `signatureFeePayer` for gasless swaps [4] |

**It also depends on:** System, SPL Token / Token-2022, Associated Token Account, Compute Budget, the DEX programs it routes into, and Pyth oracles.

## 5. Wallet & transaction flow (verified examples)
Jupiter never holds user keys. It **builds transactions and the user's wallet signs them**. Examples decoded on mainnet with `getTransaction`:

| Flow | Transaction | What happens on-chain |
|---|---|---|
| **Swap** | [`2TxVyv2f…XbF7`](https://explorer.solana.com/tx/2TxVyv2fRpwgUM3WNeEtYW7vFGkbhbaGrFwoxia95uZJDpNp6zUvyriFP8LJDbuiG15Fhhe4xqH7uXjpurk5XbF7) | v0 tx, 2 lookup tables, 2 signers: Compute Budget ×2 → create ATA → `JUP6 RouteV2` → CPI into a DEX (`BuyExactQuoteIn`) → SPL `TransferChecked` |
| **DCA fill** | [`2cXmmc8P…6itY`](https://explorer.solana.com/tx/2cXmmc8PDVp9NHymHPzcxfxUgRtRBuGQ7rtBnEhaeu63813CNVEu1URvvUcQHwpbsSjAE1qf1n2LBSwgbpnY6itY) | A keeper runs `DCA InitiateFlashFill` → `JUP6 SharedAccountsRoute` → `DCA FulfillFlashFill`. **The DCA program composes the swap router.** |
| **Limit order** | [`2FKG4vDq…PAGS`](https://explorer.solana.com/tx/2FKG4vDqQepSoLSBCtGQivNR34BoYBXyxhmW1oXrB8fX7dgAcdX8sDyW1opJgRApSoC5pRALHQzja6cRhgjnPAGS) | User signs `InitializeOrder` on Limit Order v2 (funds moved into an order vault) |
| **Perps keeper** | [`s3KrRb5K…Jrss`](https://explorer.solana.com/tx/s3KrRb5KRvzqbincjdqL1F2Fw8Dm19yt9CQJ5Nxpe315AcoN5YMgV8ohn7jCq3ZQ1VFWafbWxAMcj1LR91YJrss) | `Ed25519SigVerify` → oracle `UpdateManyWithPythLazer` / `UpdateAgPrice2` → `PERP RefreshAssetsUnderManagement` |

**Key pattern:** users sign *intents* (a swap, an order, a DCA plan). Jupiter's **off-chain keepers** later submit the transactions that fill them, and the on-chain programs enforce the rules.

## 6. Tokens & on-chain assets
Any SPL / Token-2022 token it can route · **JUP** (governance, used in the Jupiter DAO/staking) · **JLP** (perps liquidity: holders earn trading fees and take the other side of traders) · **jupSOL** (liquid-staked SOL) · **JupUSD** (Jupiter's stablecoin) · jlTokens from Jupiter Lend. [1]

## 7. APIs, SDKs & infrastructure
- **Swap API v2** (`/order` + `/execute`, `/build`), **Price API v3** (up to 50 mints per call), **Tokens API**, **Trigger / Recurring**, **Lend**, **Perps** (IDL-based) [1]
- **Developer Platform:** API key in the `x-api-key` header. Keyless use is 0.5 RPS; plans are Free 1 RPS → Pro 150 RPS. [6]
- **Transaction landing** (`tx.jup.ag`): a high-stake validator for SWQoS bandwidth, a custom TPU forwarder, and MEV protection [1]
- SDKs and tools: `@jup-ag/lend`, the Referral SDK, the Jupiter CLI, a Trading MCP server for AI agents, and the Jupiter Plugin widget [1]

## 8. Competitors & alternatives
**Aggregators:** OKX DEX aggregator, DFlow, Titan. Jupiter even lets some of these compete *inside* its own `/order`. **Perps:** Drift. **Trading terminals / bots** (Photon, Axiom, BullX) for memecoin traders. **Going direct to a DEX** (Raydium, Orca, Meteora) for single-pool trades.

## 9. Business model
- **Platform fee on `/order` swaps**, which depends on the pair [2]:

| Pair | Fee |
|---|---|
| Buying JUP / JLP / jupSOL | 0 bps |
| Stable↔stable, LST↔LST | 0 bps |
| SOL↔stable | 2 bps |
| LST↔stable | 5 bps |
| Everything else | 10 bps |
| New tokens (< 24 h old) | 50 bps |

- **Referral share:** integrators set 50–255 bps, and Jupiter keeps 20% of it. [2]
- **Perps:** open, close and borrow fees. Most go to JLP holders, and the protocol keeps a share.
- **Developer Platform:** paid API tiers ($25–$500 per month). [6]
- **Other products:** Lend, token-verification fees (1,000 JUP for express verification), and the stablecoin. [1]

## 10. Main technical risks
- **Upgradeable programs:** v6 and Perps both have an upgrade authority (e.g. `CvQZZ23q…` for v6). Users trust whoever controls it. It's a single address on-chain, so you can't tell from the account alone whether it's a multisig.
- **Composability risk:** one swap CPIs into many third-party DEX programs. A bug or exploit in any routed pool affects the trade.
- **Oracle risk (Perps/JLP):** liquidations and pool value depend on keeper-pushed prices. Stale or manipulated prices directly hit JLP holders.
- **Off-chain keepers:** DCA, limit orders and perps requests need Jupiter's keepers to execute. If they go down, orders don't fill (though funds stay in program vaults).
- **Centralised API:** most apps call `api.jup.ag`, so an outage or rate limit breaks swaps across the whole ecosystem.
- **MEV and slippage** on volatile tokens, which Jupiter mitigates with RTSE slippage estimation and private landing.

## 11. What could be improved
- **Publish the upgrade-authority setup** (multisig and timelock) for each program in one place.
- **Keeper decentralisation:** let anyone fill DCA or limit orders permissionlessly, so they don't rely on one operator.
- **Fee transparency for end users** in wallets that embed Jupiter: show the platform plus referral fee as one amount.
- **Beginner and local-language UX:** the DeFi super-app is powerful but intimidating. A voice layer such as Mzia could call `/order` for "convert 20 USDC to USDT" and read the quote back in Georgian before signing.

## On-chain vs off-chain

| On-chain | Off-chain |
|---|---|
| Swap execution via `JUP6` + DEX CPIs (atomic) | Route finding (Metis engine), RFQ market-maker quotes |
| Limit-order / DCA vaults and their rules | **Keepers** that trigger fills |
| Perps positions, JLP pool, custody accounts | Price feeds signed off-chain, pushed by keepers |
| JUP / JLP / JupUSD mints, balances | APIs, rate limits, analytics, tx landing (`tx.jup.ag`) |

**Key insight:** Jupiter is **genuinely on-chain infrastructure** (it has its own programs and pools, and the rules are enforced by code), but **it only works in practice with its off-chain routing engine and keepers**. That's the opposite of Phantom, where almost everything valuable is off-chain.

## Sources
1. Jupiter Developer Docs index (products, Price API, tx landing, JupUSD, Lend, CLI/MCP): https://dev.jup.ag/docs/llms.txt · https://developers.jup.ag/docs
2. Swap API: Order & Execute (routing engines, **fee table**, referral fees): https://developers.jup.ag/docs/swap/order-and-execute.md
3. Swap API: Build (raw instructions, no swap fee): https://developers.jup.ag/docs/swap/build/index.md
4. Gasless swaps (`signatureFeePayer`): https://developers.jup.ag/docs/swap/advanced/gasless.md
5. Perps program accounts (Pool, Custody, Position): https://developers.jup.ag/docs/perps/index.md
6. Developer Platform rate limits and plans: https://developers.jup.ag/docs/portal/rate-limits.md
7. Program addresses, token supplies, upgrade authorities and all example transactions were verified directly via Solana mainnet RPC (`getAccountInfo`, `getSignaturesForAddress`, `getTransaction`, `solana program show`) on 2026-09-28.
