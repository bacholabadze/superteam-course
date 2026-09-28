# Assignment 1 — Solana CLI & First Transaction (Devnet)

## Result

| | |
|---|---|
| Network | Solana **Devnet** |
| Public wallet address | `Ga9MUnanEefrdPx1P8qGLPrC1gQrVdgLS3np2zz6wrrh` |
| Receiver | `HofcehzMutPdNk6LyYuXD4U4mf1YVT9Ec6d9ZNkEKsJs` |
| Amount | 0.1 SOL |
| Fee | 0.000005 SOL (5,000 lamports) |
| Status | ✅ Success — Finalized |
| Transaction signature | `5Fib7N12VdxRPgt8us4BckHzABXQUjiWKgpWe8cc9adcz2syTFz5wESBvqRYdarSQTQ8wQysJeME3fUHq2ZCSUBe` |
| Explorer | [View on Solana Explorer](https://explorer.solana.com/tx/5Fib7N12VdxRPgt8us4BckHzABXQUjiWKgpWe8cc9adcz2syTFz5wESBvqRYdarSQTQ8wQysJeME3fUHq2ZCSUBe?cluster=devnet) |

## Commands used

```bash
# 1. Install the Solana CLI (Anza / Agave)
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
solana --version                       # solana-cli 4.x

# 2. Switch to Devnet and verify
solana config set --url devnet
solana config get                      # RPC URL: https://api.devnet.solana.com

# 3. Create a new keypair (stored locally in ~/.config/solana/id.json — never shared or committed)
solana-keygen new

# 4. Show the public address
solana address

# 5. Get Devnet SOL
solana airdrop 2                       # was rate-limited, so we used the official faucet:
                                       # https://faucet.solana.com (Devnet, 2.5 SOL)

# 6. Check the balance
solana balance                         # 2.5 SOL

# 7. Send a small amount of SOL to another address
solana transfer HofcehzMutPdNk6LyYuXD4U4mf1YVT9Ec6d9ZNkEKsJs 0.1 --allow-unfunded-recipient

# 8. Inspect the transaction from the terminal
solana confirm -v 5Fib7N12VdxRPgt8us4BckHzABXQUjiWKgpWe8cc9adcz2syTFz5wESBvqRYdarSQTQ8wQysJeME3fUHq2ZCSUBe
```

## Explorer check

| Field | Value |
|---|---|
| Sender (fee payer, signer) | `Ga9MUnanEefrdPx1P8qGLPrC1gQrVdgLS3np2zz6wrrh` — 2.5 → 2.399995 SOL |
| Receiver | `HofcehzMutPdNk6LyYuXD4U4mf1YVT9Ec6d9ZNkEKsJs` — 0 → 0.1 SOL |
| Program | System Program `11111111111111111111111111111111` — `Transfer { lamports: 100000000 }` |
| Fee | 0.000005 SOL |
| Status | Success, Finalized (slot 505028677) |

## Summary

დავაყენეთ Solana CLI, გადავერთეთ Devnet ქსელზე, შევქმენით ახალი keypair და გამოვიტანეთ მისი public address. რადგან CLI-დან airdrop დროებით შეზღუდული იყო, wallet ოფიციალური Solana faucet-ის გამოყენებით 2.5 test SOL-ით შევავსეთ. შემდეგ მეორე მისამართზე გავგზავნეთ 0.1 SOL და Solana Explorer-ში გადავამოწმეთ გამგზავნი, მიმღები, თანხა, საკომისიო და ტრანზაქციის სტატუსი. Transaction signature არის გამგზავნის Ed25519 ხელმოწერა კონკრეტულ ტრანზაქციაზე და, რადგან ის უნიკალურია, მისი გამოყენება შესაძლებელია როგორც transaction ID, რომლითაც ნებისმიერ ადამიანს შეუძლია blockchain-ზე ტრანზაქციის მოძებნა და გადამოწმება.

> 🔐 The private key / keypair file is stored only on the local machine and is not part of this repository.
