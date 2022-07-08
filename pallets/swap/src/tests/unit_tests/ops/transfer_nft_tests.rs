use super::*;

#[test]
fn transfer_nft_works() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //TransferNFT from accountIndex 2 to accountIndex 3
        origin = 2u64;
        let recipient = 3u32;
        nonce = 1;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 3u32);
        assert_eq!(nft.1, U256::from(0));
        assert_eq!(nft.2, None);
    })
}

#[test]
fn transfer_nft_account_not_exists() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //TransferNFT from accountIndex 4 to accountIndex 3
        //Not setKey for accountIndex 4
        origin = 4u64;
        let recipient = 3u32;
        nonce = 1;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        // There is no secret_key_4, so use secret_key_3 here
        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn transfer_nft_invalid_nftindex() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let mut nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //TransferNFT from accountIndex 2 to accountIndex 3
        origin = 2u64;
        //nft_id 1u32 << 20 exceeds the range 20 bits
        nft_id = 1u32 << 20;
        let recipient = 3u32;
        nonce = 1;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::InvalidNFTIndex);

        //TransferNFT from accountIndex 2 to accountIndex 3
        //nft_id should not be 0
        nft_id = 0;

        pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn transfer_nft_invalid_account() {
    new_test_ext().execute_with(|| {
         //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 4
        origin = 4u64;
        let secret_key_4 = [6u8;32];
        let pub_key_4 = BabyJubjub::pubkey_from_secretkey(&secret_key_4).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_4));

        //TransferNFT from accountIndex 2 to accountIndex 4
        origin = 2u64;
        //Not setKey for accountIndex 3
        let mut recipient = 4u32;
        nonce = 1;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::InvalidAccount);

        //TransferNFT from accountIndex 2 to accountIndex 4
        //recipient should no be zero
        recipient = 0u32;

        pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::InvalidAccount);
    })
}

#[test]
fn transfer_nft_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //TransferNFT from accountIndex 2 to accountIndex 3
        origin = 2u64;
        let recipient = 3u32;
        //True nonce is 1u64
        nonce = 2u64;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn transfer_nft_invalid_signature() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //TransferNFT from accountIndex 2 to accountIndex 3
        origin = 2u64;
        let recipient = 3u32;
        nonce = 1;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        //command_sign_formatted use recipient 3u32
        let recipient = 2u32;

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::InvalidSignature);
    })
}


#[test]
fn transfer_nft_is_not_owner() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 1
        origin = 1u64;
        let secret_key_1 = [3u8; 32];
        let pub_key_1 = BabyJubjub::pubkey_from_secretkey(&secret_key_1).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

        //SetKey for accountIndex 2
        origin = 2u64;
        let secret_key_2 = [4u8;32];
        let pub_key_2 = BabyJubjub::pubkey_from_secretkey(&secret_key_2).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let mut account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //DepositNFT for accountIndex 2, caller is accountIndex 1
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), command_sign_formatted, account_index, nft_id, l1_tx_hash, nonce));

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));


        //TransferNFT from accountIndex 1 to accountIndex 3
        //Caller should be owner of nft
        origin = 1u64;
        account_index = 1u32;
        let recipient = 3u32;
        nonce = 3;

        let mut pad_recipient = [0u8; 32];
        pad_recipient[28..].copy_from_slice(&recipient.to_be_bytes());
        command = [0u8; 81];
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&pad_recipient);

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), command_sign_formatted, nft_id, recipient, nonce), Error::<Test>::IsNotOwner);
    })
}
