// Calls the deployed mzia_contacts program on Devnet.
//
//   npm run client -- add deda <ADDRESS>   save a trusted contact
//   npm run client -- show deda            read it back from chain
//   npm run client -- remove deda          close it, rent refunded
//
// Signs with the Solana CLI keypair (~/.config/solana/id.json) unless
// ANCHOR_WALLET points elsewhere. The keypair never leaves this machine.
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import { AnchorProvider, Program, Wallet } from "@anchor-lang/core";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import type { MziaContacts } from "../idl/mzia_contacts.ts";

const idl = JSON.parse(readFileSync(new URL("../idl/mzia_contacts.json", import.meta.url), "utf8")) as MziaContacts;

const rpcUrl = process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com";
const walletPath = process.env.ANCHOR_WALLET ?? `${homedir()}/.config/solana/id.json`;
const owner = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(readFileSync(walletPath, "utf8"))));

const provider = new AnchorProvider(new Connection(rpcUrl, "confirmed"), new Wallet(owner), {
  commitment: "confirmed",
});
const program = new Program<MziaContacts>(idl, provider);

const explorer = (sig: string) => `https://explorer.solana.com/tx/${sig}?cluster=devnet`;

function contactPda(name: string): PublicKey {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("contact"), owner.publicKey.toBuffer(), Buffer.from(name)],
    program.programId,
  )[0];
}

async function main(): Promise<void> {
  const [command, name, address] = process.argv.slice(2);
  if (!command || !name) throw new Error("usage: add <name> <address> | show <name> | remove <name>");

  const contact = contactPda(name);
  console.log(`program: ${program.programId.toBase58()}`);
  console.log(`owner:   ${owner.publicKey.toBase58()}`);
  console.log(`contact: ${contact.toBase58()} (PDA)`);

  if (command === "add") {
    if (!address) throw new Error("add needs an address");
    const sig = await program.methods
      .addContact(name, new PublicKey(address))
      .accounts({ owner: owner.publicKey })
      .rpc();
    console.log(`added '${name}' -> ${address}\nsignature: ${sig}\n${explorer(sig)}`);
  } else if (command === "remove") {
    const sig = await program.methods
      .removeContact()
      .accountsPartial({ owner: owner.publicKey, contact })
      .rpc();
    console.log(`removed '${name}'\nsignature: ${sig}\n${explorer(sig)}`);
  } else if (command === "show") {
    const state = await program.account.contact.fetch(contact);
    console.log(`'${state.name}' -> ${state.address.toBase58()}`);
  } else {
    throw new Error(`unknown command: ${command}`);
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
