use super::*;

#[test]
fn finalize_nft_works() {
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
        let mut amount = U256::from(10);
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

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        origin = 2u64;
        account_index = 2u32;
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 3u32);
        assert_eq!(nft.1, U256::from(0));
        assert_eq!(nft.2, None);
        assert_eq!(BalanceMap::get((2u32, token_index)), U256::from(11));
    })
}

#[test]
fn finalize_nft_account_not_exists() {
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
        let mut amount = U256::from(10);
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

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        //Not setKey for accountIndex 4
        origin = 4u64;
        nonce = 0u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        // There is no secret_key_4, use secret_key_3 here
        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn finalize_nft_invalid_nftindex() {
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
        let mut amount = U256::from(10);
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

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        //There is no bidder, so nft_id 4u32 is invalid
        origin = 2u64;
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::InvalidNFTIndex);

        //SetKey for accountIndex 3
        origin = 3u64;
        let secret_key_3 = [5u8;32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        origin = 2u64;
        //1u32 << 20 exceeds the range 20 bits
        nft_id = 1u32 << 20;
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::InvalidNFTIndex);

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        //nft_id should not be zero
        nft_id = 0;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn finalize_nft_nonce_inconsistent() {
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
        let mut amount = U256::from(10);
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

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

         //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        origin = 2u64;
        //True nonce is 1u64
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn finalize_nft_invalid_signature() {
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
        let mut amount = U256::from(10);
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

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        origin = 2u64;
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        //command_sign_formatted[0] is OP_Finalize_NFT
        command_sign_formatted[0] = 1;

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn finalize_nft_is_not_owner() {
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
        let mut amount = U256::from(10);
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

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        //FinalizeNFT for accountIndex 1, caller is accountIndex 1
        //Owner of nft is accountIndex 2
        origin = 1u64;
        account_index = 1u32;
        nonce = 4u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::IsNotOwner);
    })
}

#[test]
fn finalize_nft_balance_overflow() {
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
        let mut amount = U256::from(10);
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

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        origin = 3u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        //Deposit (U256::from(1) << 250) - 11 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        account_index = 2u32;
        amount = (U256::from(1) << 250) - 11;
        l1_tx_hash = U256::from(2);
        nonce = 4u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_1);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //FinalizeNFT for accountIndex 2, caller is accountIndex 2
        //Can not add any amount to balance of accountIndex 2 anymore
        origin = 2u64;
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), command_sign_formatted, nft_id, nonce), Error::<Test>::BalanceOverflow);
    })
}
