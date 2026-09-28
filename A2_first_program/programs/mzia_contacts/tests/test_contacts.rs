
use {
    anchor_lang::{
        prelude::Pubkey,
        solana_program::{instruction::Instruction, system_program},
        AccountDeserialize, InstructionData, ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

fn setup() -> (LiteSVM, Keypair) {
    let owner = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/mzia_contacts.so"
    ));
    svm.airdrop(&owner.pubkey(), 1_000_000_000).unwrap();
    svm.add_program(mzia_contacts::id(), bytes).unwrap();
    (svm, owner)
}

fn contact_pda(owner: &Pubkey, name: &str) -> Pubkey {
    Pubkey::find_program_address(
        &[mzia_contacts::constants::CONTACT_SEED, owner.as_ref(), name.as_bytes()],
        &mzia_contacts::id(),
    )
    .0
}

fn send(svm: &mut LiteSVM, owner: &Keypair, instruction: Instruction) -> bool {
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&owner.pubkey()), &blockhash);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[owner]).unwrap();
    svm.send_transaction(tx).is_ok()
}

fn add_contact_ix(owner: &Pubkey, name: &str, address: Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        mzia_contacts::id(),
        &mzia_contacts::instruction::AddContact { name: name.to_string(), address }.data(),
        mzia_contacts::accounts::AddContact {
            owner: *owner,
            contact: contact_pda(owner, name),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}

#[test]
fn test_add_and_remove_contact() {
    let (mut svm, owner) = setup();
    let mom = Keypair::new().pubkey();
    let contact = contact_pda(&owner.pubkey(), "deda");

    assert!(send(&mut svm, &owner, add_contact_ix(&owner.pubkey(), "deda", mom)));

    let account = svm.get_account(&contact).unwrap();
    let mut data: &[u8] = &account.data;
    let state = mzia_contacts::state::Contact::try_deserialize(&mut data).unwrap();
    assert_eq!(state.owner, owner.pubkey());
    assert_eq!(state.address, mom);
    assert_eq!(state.name, "deda");

    let instruction = Instruction::new_with_bytes(
        mzia_contacts::id(),
        &mzia_contacts::instruction::RemoveContact {}.data(),
        mzia_contacts::accounts::RemoveContact { owner: owner.pubkey(), contact }
            .to_account_metas(None),
    );
    assert!(send(&mut svm, &owner, instruction));

    // Closed accounts are purged, or left with zero lamports.
    assert!(svm.get_account(&contact).map_or(true, |a| a.lamports == 0));
}

#[test]
fn test_rejects_self_contact() {
    let (mut svm, owner) = setup();
    assert!(!send(&mut svm, &owner, add_contact_ix(&owner.pubkey(), "me", owner.pubkey())));
}
