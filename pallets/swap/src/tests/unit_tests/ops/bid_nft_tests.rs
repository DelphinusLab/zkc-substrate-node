use super::*;

#[test]
fn bid_nft_works_has_bidder() {
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

        //SetKey for accountIndex 4
        origin = 4u64;
        let secret_key_4 = [6u8;32];
        let pub_key_4 = BabyJubjub::pubkey_from_secretkey(&secret_key_4).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_4));

        //Deposit 10 into accountIndex 4, caller is accountIndex 1
        origin = 1u64;
        account_index = 4u32;
        amount = U256::from(10);
        l1_tx_hash = U256::from(8);
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

        //BidNFT for accountIndex 4, caller is accountIndex 4
        origin = 4u64;
        amount = U256::from(2);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_4);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 2u32);
        assert_eq!(nft.1, U256::from(2));
        assert_eq!(nft.2, Some(4u32));
        assert_eq!(BalanceMap::get((3u32, token_index)), U256::from(10));
        assert_eq!(BalanceMap::get((4u32, token_index)), U256::from(8));
    })
}

#[test]
fn bid_nft_works_no_bidder() {
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

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 2u32);
        assert_eq!(nft.1, U256::from(1));
        assert_eq!(nft.2, Some(3u32));
        assert_eq!(BalanceMap::get((3u32, token_index)), U256::from(9));
    })
}

#[test]
fn bid_nft_account_not_exists() {
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

        //BidNFT for accountIndex 4, caller is accountIndex 4
        //Not set_key on accountIndex 4
        origin = 4u64;
        amount = U256::from(1);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        //There is no secret_key_4, so use secret_key_3 here
        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn bid_nft_invalid_nftindex() {
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
        //exceed the range 20 bits
        nft_id = 1u32 << 20;
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

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::InvalidNFTIndex);

        //BidNFT for accountIndex 3, caller is accountIndex 3
        //nft_id should not be zero
        nft_id = 0u32;

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

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn bid_nft_invalid_amount() {
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
        //U256::from(1) << 250 exceeds the range 250 bits
        amount = U256::from(1) << 250;
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

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::InvalidAmount);

        //BidNFT for accountIndex 3, caller is accountIndex 3
        amount = U256::from(1);

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

        //BidNFT for accountIndex 3, caller is accountIndex 3
        //Amount must be greater than last biddingAmount
        amount = U256::from(1);

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

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::InvalidAmount);
    })
}

#[test]
fn bid_nft_nonce_inconsistent() {
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
        //True nonce is 1u64
        nonce = 2u64;

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

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn bid_nft_invalid_signature() {
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

        //command_sign_formatted use amount U256::from(1)
        amount = U256::from(2);

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn bid_nft_invalid_nftindex_no_owner() {
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
        let mut amount = U256::from(10);
        let l1_tx_hash = U256::from(12);
        let nonce = 1u64;

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

        //BidNFT for accountIndex 2, caller is accountIndex 2
        origin = 2u64;
        //Not depositNFT
        let nft_id = 4u32;
        amount = U256::from(1);

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn bid_nft_has_bidder_balance_overflow() {
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

        //Deposit (U256::from(1) << 250) - 10 into accountIndex 3, caller is accountIndex 1
        origin = 1u64;
        account_index = 3u32;
        amount = (U256::from(1) << 250) - 10;
        l1_tx_hash = U256::from(7);
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

        //SetKey for accountIndex 4
        origin = 4u64;
        let secret_key_4 = [6u8;32];
        let pub_key_4 = BabyJubjub::pubkey_from_secretkey(&secret_key_4).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_4));

        //Deposit 10 into accountIndex 4, caller is accountIndex 1
        origin = 1u64;
        account_index = 4u32;
        amount = U256::from(10);
        l1_tx_hash = U256::from(8);
        nonce = 5u64;

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

        //BidNFT for accountIndex 4, caller is accountIndex 4
        //Add 1 to balance of accountIndex 3
        origin = 4u64;
        amount = U256::from(2);
        nonce = 1u64;

        command = [0u8; 81];
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_4);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::BalanceOverflow);
    })
}

#[test]
fn bid_nft_balance_not_enough() {
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
        //Current balance of accountIndex 3 is 10
        amount = U256::from(11);
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

        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), command_sign_formatted, nft_id, amount, nonce), Error::<Test>::BalanceNotEnough);
    })
}
