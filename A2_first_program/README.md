# Assignment 2 — First Solana Program: `mzia_contacts`

A tiny Anchor program that stores a user's **trusted contacts** on-chain: a short name such as `deda` ("mom") mapped to a wallet address.
It's the on-chain building block for **Mzia**, our voice-first Georgian wallet. When a user says *"send 10 USDT to mom"*, Mzia can resolve "mom" to a contact the user saved and verified earlier, instead of a pasted address that might have been swapped.

## Devnet deployment

| | |
|---|---|
| Program ID | [`5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H`](https://explorer.solana.com/address/5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H?cluster=devnet) |
| Deploy signature | [`6545PRNi…zEx1AJP`](https://explorer.solana.com/tx/6545PRNioax5YDrmR8BgBrrE5ecFvepeoUkLeGp9nq9amnK9sPpKrkfyvfxqaLYN7ishC1SozZ5coQ8ahzEx1AJP?cluster=devnet) |
| Upgrade authority | `Ga9MUnanEefrdPx1P8qGLPrC1gQrVdgLS3np2zz6wrrh` |
| Framework | Anchor 1.2.0, Solana CLI 4.x, SBPF v0 |

### Successful interactions

| Instruction | Signature | Result |
|---|---|---|
| `add_contact("deda", Hofc…KsJs)` | [`2f8nfC9F…6VwG1ZiK`](https://explorer.solana.com/tx/2f8nfC9FBbCGkDXBEQkPaNe9t9EvAMt4Qt2FTnyDYeVT644PLuVXakyTdUo2xWLkB89wodL3KitQRUZk6VwG1ZiK?cluster=devnet) | ✅ PDA `HmBkzgyN…YFEini` created |
| `add_contact("bebia", Hofc…KsJs)` | [`2zph274p…HAKb`](https://explorer.solana.com/tx/2zph274pyidrHBE2GzTu8DJDrc1qB6L7vDBP7DaRWxSLbHiMCCidtw29rPpgc27bqvWaBcyadCvJueUMLBGkHAKb?cluster=devnet) | ✅ PDA `9wvoZHAC…LyyvJ` created |
| `remove_contact()` on `bebia` | [`4bPZE34F…BD9J`](https://explorer.solana.com/tx/4bPZE34Fvt1Q6HTTaUvkgJDerXte7CX1YPKbx2s5KeFZNKXHKTJKvqdZTUmPGKNgujcsw6QWUiM73X9DUZR7BD9J?cluster=devnet) | ✅ PDA closed, rent refunded |

Program log from the first `add_contact`:

```
Program 5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H invoke [1]
Program log: Instruction: AddContact
Program 11111111111111111111111111111111 invoke [2]
Program 11111111111111111111111111111111 success
Program log: Mzia: saved trusted contact 'deda' -> HofcehzMutPdNk6LyYuXD4U4mf1YVT9Ec6d9ZNkEKsJs
Program 5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H consumed 15196 of 200000 compute units
Program 5gGkH3UrsLWEfJ1EZouzpUeH1UZjiRTkaP7aHHLcin5H success
```

## Accounts & instructions

**State: `Contact`** (one account per saved contact)

| Field | Type | Meaning |
|---|---|---|
| `owner` | `Pubkey` | Wallet that saved the contact |
| `address` | `Pubkey` | The contact's wallet address |
| `name` | `String` (≤ 32 bytes) | Short label, e.g. `deda` |
| `bump` | `u8` | PDA bump seed |

The account is a **PDA** derived from `["contact", owner, name]`. Every owner gets their own address book, and a name is unique within it.

**`add_contact(name, address)`**

| Account | Signer | Writable | Role |
|---|---|---|---|
| `owner` | ✅ | ✅ | Signs the transaction and pays the fee plus the account's rent |
| `contact` | | ✅ | New PDA, created (`init`) by the program through the System Program |
| `system_program` | | | Creates the account |

Checks: the name must be 1–32 bytes, and the contact can't be the owner's own address.

**`remove_contact()`**

| Account | Signer | Writable | Role |
|---|---|---|---|
| `owner` | ✅ | ✅ | Must be the contact's owner (`has_one = owner`); receives the refunded rent |
| `contact` | | ✅ | Closed (`close = owner`) |

The PDA seeds include the signer's key, so no one else can create or delete contacts in your book.

`owner` wallet არის ერთადერთი signer, რომელიც ტრანზაქციას ავტორიზაციას უკეთებს და ახალი account-ის შექმნის ხარჯს იხდის. `add_contact` ქმნის პროგრამის მიერ მართულ PDA account-ს, სადაც ინახება contact-ის სახელი და wallet address. `remove_contact` ამ account-ს ხურავს და მისთვის გამოყოფილ rent-ს owner-ს უბრუნებს. PDA მისამართი owner-ის public key-ისა და contact-ის სახელის მიხედვით წარმოიქმნება, ამიტომ თითოეულ მომხმარებელს საკუთარი contact book აქვს და მის ჩანაწერებს მხოლოდ თავად მართავს.

## Project layout

```
programs/mzia_contacts/src/
  lib.rs                         # program entry: add_contact, remove_contact
  state.rs                       # Contact account
  constants.rs, error.rs
  instructions/add_contact.rs
  instructions/remove_contact.rs
programs/mzia_contacts/tests/    # LiteSVM tests
client/call.ts                   # Devnet client (TypeScript, @anchor-lang/core)
idl/                             # generated IDL + TS types used by the client
```

## How to run

Prerequisites: Rust, [Solana CLI](https://solana.com/docs/intro/installation), Anchor 1.2 (`avm install 1.2.0`), Node 20+.

```bash
npm install

# Build (SBPF v0 for compatibility; Anchor 1.2 defaults to v3) and run the tests
npm run build
npm test                       # 2 LiteSVM tests: add → read → remove, reject self-contact

# Deploy to Devnet (needs ~1.7 SOL while deploying; ~0.82 SOL stays locked as rent)
solana config set --url devnet
solana program deploy target/deploy/mzia_contacts.so \
  --program-id target/deploy/mzia_contacts-keypair.json

# Call the program (signs with ~/.config/solana/id.json, or set ANCHOR_WALLET)
npm run client -- add deda <ADDRESS>
npm run client -- show deda
npm run client -- remove deda
```

To use your own deployment, run `anchor keys sync` before building so the program ID matches your keypair.

> 🔐 Keypairs (`~/.config/solana/id.json`, `target/deploy/*-keypair.json`) and `.env` files are git-ignored and never committed.
