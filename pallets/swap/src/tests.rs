use super::*;
use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};
use std::fs::File;
use std::io::BufReader;
use std::process;

fn get_public_key(name: &str) -> [u8; 32] {
    let path = "./src/unit_tests/input/pk".to_owned() + name +".json";
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("open failed: {}", e);
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    let pub_key: [u8; 32] = match serde_json::from_reader(reader) {
        Ok(c) => c,
        Err(e) => {
            println!("Read file error: {}", e);
            process::exit(1);
        }
    };
    pub_key
}

fn get_sign(name: &str) -> [u8; 64] {
        let mut path = "./src/unit_tests/input/".to_owned() + name +"_1.json";
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let mut reader = BufReader::new(file);
        let sign_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/".to_owned() + name +"_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign: [u8; 64] = [0; 64];
        sign[..32].copy_from_slice(&sign_1);
        sign[32..].copy_from_slice(&sign_2);
        sign
}

#[test]
// set key on accountIndex 0
fn set_key_works() {
    new_test_ext().execute_with(|| {
        let pub_key: [u8; 32] = get_public_key("0");
        let origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        assert_eq!(get_account_index::<Test>(&0u64).unwrap(), 0u32);
    })
}

#[test]
fn set_key_invalid_key() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        // change first value of public key to 9u8 which is diffrent from the original value to verity InvalidKey error
        pub_key[0] = 9u8;
        let origin = 0u64;
        assert_noop!(SwapModule::set_key(Origin::signed(origin), pub_key), Error::<Test>::InvalidKey);
    })
}

#[test]
fn set_key_account_exists() {
    new_test_ext().execute_with(|| {
        let pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // execute set_key on same origin 0u64 to verify AccountExists error
        origin = 0u64;
        assert_noop!(SwapModule::set_key(Origin::signed(origin), pub_key), Error::<Test>::AccountExists);
    })
}

#[test]
// deposit on accountIndex 2, accountIndex 1 is the caller which is also admin
fn deposit_works() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32; // set to 1u32 is because the tokenIndex in zkp tools is set to 1 temporarily
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        assert_eq!(BalanceMap::get((&account_index, token_index)), amount);
    })
}

#[test]
fn deposit_noaccess() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // deposit on accountIndex 2, set the origin to 0u64 which is not admin, so it has no access to it
        origin = 0u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::NoAccess);
    })
}

#[test]
// not set_key on accountIndex 2 to verity AccountNotExists error
fn deposit_account_not_exists() {
    new_test_ext().execute_with(|| {
        let origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn deposit_invalid_tokenindex() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        // set token_index to 1u32 << 10 to exceed the range
        let token_index = 1u32 << 10;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidTokenIndex);
    })
}

#[test]
// omit set_key on accountIndex 2 on purpose to let account_index less than AccountIndexCount::get()
fn deposit_invalid_account() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 3u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidAccount);
    })
}

#[test]
fn deposit_l1tx_exists() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        // execute deposit twice with same l1_tx_hash U256::from(0) to verify L1TXExists error
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::L1TXExists);
    })
}

#[test]
fn deposit_invalid_amount() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        // set amount to U256::from(10) << 250 to exceed the range of 250 bits
        let amount = U256::from(10) << 250;
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidAmount);
    })
}

#[test]
fn deposit_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        // the true nonce is 1u64, set nonce to 2u64 to verify NonceInconsistent error
        let nonce = 2u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn deposit_invalid_signature() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        // set token_index to 0u32 which is different from tokenIndex in zkp but use the sign_deposit2 generated by zkp tools
        let token_index = 0u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn deposit_balance_overflow() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        // deposit on accountIndex 2 twice, the balance of 2 exceed the range of balance
        let sign_deposit2_balance_overflow: [u8; 64] = get_sign("deposit2_balance_overflow");
        amount = (U256::from(1) << 250) - 10;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_noop!(SwapModule::deposit(Origin::signed(origin), sign_deposit2_balance_overflow, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::BalanceOverflow);
    })
}

#[test]
// execute deposit_nft on accountIndex 2, account 1 is the caller of deposit and deposit_nft
fn deposit_nft_works() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, account_index);
        assert_eq!(nft.1, U256::from(0));
        assert_eq!(nft.2, None);
    })
}

#[test]
// not set_key on accountIndex 0, so the origin 0u64 is invalid
fn deposit_nft_account_not_exists() {
    new_test_ext().execute_with(|| {
        let origin = 0u64;
        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let account_index = 2u32;
        let nft_id = 4u32;
        let l1_tx_hash = U256::from(1);
        let nonce = 1u64;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn deposit_nft_invalid_account() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        // not setkey on account_index 3u32, so it is invalid
        let account_index = 3u32;
        let nft_id = 4u32;
        let l1_tx_hash = U256::from(1);
        let nonce = 2u64;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::InvalidAccount);
    })
}

#[test]
fn deposit_nft_invalid_nftindex() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        // nft_id exceeds the range 20 bits
        origin = 1u64;
        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let mut nft_id = 1u32 << 20;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::InvalidNFTIndex);

        // nft_id should not be zero
        nft_id = 0;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::InvalidNFTIndex);

        nft_id = 4u32;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        // execute deposit_nft twice with the same nft_id 4u32. there shoud be no owner before deposit_nft
        let sign_deposit_nft_checked_empty: [u8; 64] = get_sign("deposit_nft_checked_empty");
        l1_tx_hash = U256::from(2);
        nonce = 3u64;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft_checked_empty, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn deposit_nft_l1tx_exists() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        // use the same l1_tx_hash U256::from(1) which is not allowed
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::L1TXExists);
    })
}

#[test]
fn deposit_nft_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        // the true nonce is 2u64, set nonce to 3u64 to verify NonceInconsistent error
        nonce = 3u64;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn deposit_nft_invalid_signature() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        // 3u32 is diffrent from nftIndex in zkp which is 4u32
        let nft_id = 3u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_noop!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
// withdraw_nft on accountIndex 2 after bid_nft on accountIndex 3
fn withdraw_nft_works_has_bidder() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 2u64;
        let sign_withdraw_nft_has_bidder: [u8; 64] = get_sign("withdraw_nft_has_bidder");
        let l1account = U256::from(3);
        nonce = 1u64;
        assert_ok!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_has_bidder, nft_id, l1account, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 0);
        assert_eq!(nft.1, U256::from(0));
        assert_eq!(nft.2, None);
        assert_eq!(BalanceMap::get((&account_index, token_index)), U256::from(10));
    })
}

#[test]
// withdraw_nft on accountIndex 2 whithout bidder
fn withdraw_nft_works_no_bidder() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        origin = 2u64;
        let sign_withdraw_nft_no_bidder: [u8; 64] = get_sign("withdraw_nft_no_bidder");
        let l1account = U256::from(3);
        nonce = 1u64;
        assert_ok!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_no_bidder, nft_id, l1account, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 0);
        assert_eq!(nft.1, U256::from(0));
        assert_eq!(nft.2, None);
    })
}

#[test]
fn withdraw_nft_account_not_exists() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        // not set_key on origin 3u64
        origin = 3u64;
        let sign_withdraw_nft_no_bidder: [u8; 64] = get_sign("withdraw_nft_no_bidder");
        let l1account = U256::from(3);
        nonce = 2u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_no_bidder, nft_id, l1account, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn withdraw_nft_invalid_nftindex() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let mut nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        let sign_withdraw_nft_no_bidder: [u8; 64] = get_sign("withdraw_nft_no_bidder");
        // nft_id exceeds the range 20 bits
        nft_id = 1u32 << 20;
        let l1account = U256::from(3);
        nonce = 2u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_no_bidder, nft_id, l1account, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn withdraw_nft_l1account_overflow() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        let sign_withdraw_nft_no_bidder: [u8; 64] = get_sign("withdraw_nft_no_bidder");
        // l1account exceeds the range 250 bits
        let l1account = U256::from(1) << 250;
        nonce = 2u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_no_bidder, nft_id, l1account, nonce), Error::<Test>::L1AccountOverflow);
    })
}

#[test]
fn withdraw_nft_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        let origin = 2u64;
        let sign_withdraw_nft_no_bidder: [u8; 64] = get_sign("withdraw_nft_no_bidder");
        let l1account = U256::from(3);
        // the true nonce is 1u64, set nonce to 3u64 to verify NonceInconsistent error
        nonce = 3u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_no_bidder, nft_id, l1account, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn withdraw_nft_invalid_signature() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        origin = 2u64;
        let sign_withdraw_nft_no_bidder: [u8; 64] = get_sign("withdraw_nft_no_bidder");
        // true l1account generated sign_withdraw_nft_no_bidder is U256::from(3)
        let l1account = U256::from(2);
        nonce = 1u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_no_bidder, nft_id, l1account, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn withdraw_nft_is_not_owner() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        // caller of withdraw_nft should be the owner 2 rather than the origin 1
        origin = 1u64;
        let sign_withdraw_nft_is_not_owner: [u8; 64] = get_sign("withdraw_nft_is_not_owner");
        let l1account = U256::from(3);
        nonce = 3u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_is_not_owner, nft_id, l1account, nonce), Error::<Test>::IsNotOwner);
    })
}

#[test]
fn withdraw_nft_has_bidder_balance_overflow() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 1u64;
        let sign_deposit3_again: [u8; 64] = get_sign("deposit3_again");
        account_index = 3u32;
        // after deposit here, accountIndex 3u32 has balance (U256::from(1) << 250) - 1. It cannot
        // add balance anymore
        amount = (U256::from(1) << 250) - 10;
        l1_tx_hash = U256::from(9);
        nonce = 4u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3_again, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 2u64;
        let sign_withdraw_nft_has_bidder: [u8; 64] = get_sign("withdraw_nft_deposit3_again");
        let l1account = U256::from(3);
        nonce = 1u64;
        assert_noop!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft_has_bidder, nft_id, l1account, nonce), Error::<Test>::BalanceOverflow);
    })
}

#[test]
// change the owner of nft from 2 to 3
fn transfer_nft_works() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 2u64;
        let sign_transfer_nft: [u8; 64] = get_sign("transfer_nft");
        let recipient = 3u32;
        nonce = 1;
        assert_ok!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 3u32);
        assert_eq!(nft.1, U256::from(0));
        assert_eq!(nft.2, None);
    })
}

#[test]
fn transfer_nft_account_not_exists() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // not set_key on the origin
        origin = 4u64;
        let sign_transfer_nft: [u8; 64] = get_sign("transfer_nft");
        let recipient = 3u32;
        nonce = 1;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn transfer_nft_invalid_nftindex() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let mut nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 2u64;
        let sign_transfer_nft: [u8; 64] = get_sign("transfer_nft");
        // 1u32 << 20 exceeds the range 20 bits
        nft_id = 1u32 << 20;
        let recipient = 3u32;
        nonce = 1;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::InvalidNFTIndex);

        // nft_id should not be zero
        nft_id = 0;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn transfer_nft_invalid_account() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("4");
        origin = 4u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 2u64;
        let sign_transfer_nft: [u8; 64] = get_sign("transfer_nft");
        // not set_key on 3u32, so 4u32 is invalid
        let mut recipient = 4u32;
        nonce = 1;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::InvalidAccount);

        // recipient should no be zero
        recipient = 0u32;
        nonce = 1;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::InvalidAccount);
    })
}

#[test]
fn transfer_nft_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 2u64;
        let sign_transfer_nft: [u8; 64] = get_sign("transfer_nft");
        let recipient = 3u32;
        // the true nonce is 1u64, set nonce to 2u64 to verify NonceInconsistent error
        nonce = 2u64;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn transfer_nft_invalid_signature() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 2u64;
        let sign_transfer_nft: [u8; 64] = get_sign("transfer_nft");
        // recipient generated sign_transfer_nft is 3u32
        let recipient = 2u32;
        nonce = 1;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce), Error::<Test>::InvalidSignature);
    })
}


#[test]
fn transfer_nft_is_not_owner() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // owner is 2 rather than 1
        origin = 1u64;
        let sign_transfer_nft_is_not_owner: [u8; 64] = get_sign("transfer_nft_is_not_owner");
        let recipient = 3u32;
        nonce = 3;
        assert_noop!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft_is_not_owner, nft_id, recipient, nonce), Error::<Test>::IsNotOwner);
    })
}

#[test]
// bid_nft on accountIndex 4 after bid_nft on accountIndex 3
fn bid_nft_works_has_bidder() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        pub_key = get_public_key("4");
        origin = 4u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit4: [u8; 64] = get_sign("deposit4");
        account_index = 4u32;
        amount = U256::from(10);
        l1_tx_hash = U256::from(8);
        nonce = 4u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit4, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 4u64;
        let sign_bid_nft_has_bidder: [u8; 64] = get_sign("bid_nft_has_bidder_balance_overflow");
        amount = U256::from(2);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft_has_bidder, nft_id, amount, nonce));

        let nft = NFTMap::get(&nft_id);
        assert_eq!(nft.0, 2u32);
        assert_eq!(nft.1, U256::from(2));
        assert_eq!(nft.2, Some(4u32));
        assert_eq!(BalanceMap::get((3u32, token_index)), U256::from(10));
        assert_eq!(BalanceMap::get((4u32, token_index)), U256::from(8));
    })
}

#[test]
// bid_nft on accountIndex 3
fn bid_nft_works_no_bidder() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

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
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        // not set_key on accountIndex 4
        origin = 4u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn bid_nft_invalid_nftindex() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let mut nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        // exceed the range 20 bits
        nft_id = 1u32 << 20;
        amount = U256::from(1);
        nonce = 1u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::InvalidNFTIndex);

        // nft_id should not be zero
        nft_id = 0u32;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn bid_nft_invalid_amount() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        // exceed the range 250 bits
        amount = U256::from(1) << 250;
        nonce = 1u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::InvalidAmount);

        amount = U256::from(1);
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        // amount must be greater than last biddingAmount
        amount = U256::from(1);
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::InvalidAmount);
    })
}

#[test]
fn bid_nft_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        // the true nonce is 1u64, set nonce to 2u64 to verify NonceInconsistent error
        nonce = 2u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn bid_nft_invalid_signature() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        // amount generated sign_bid_nft is U256::from(1)
        amount = U256::from(2);
        nonce = 1u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
// bid_nft must be executed after deposit_nft
fn bid_nft_invalid_nftindex_no_owner() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2_no_owner: [u8; 64] = get_sign("deposit2_no_owner");
        let account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let l1_tx_hash = U256::from(12);
        let nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2_no_owner, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 2u64;
        let sign_bid_nft_no_owner: [u8; 64] = get_sign("bid_nft_no_owner");
        // nft_id 4u32 can get owner 0
        let nft_id = 4u32;
        amount = U256::from(1);
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft_no_owner, nft_id, amount, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn bid_nft_has_bidder_balance_overflow() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 1u64;
        let sign_deposit3_balance_overflow: [u8; 64] = get_sign("deposit3_balance_overflow");
        account_index = 3u32;
        amount = (U256::from(1) << 250) - 10;
        l1_tx_hash = U256::from(7);
        nonce = 4u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3_balance_overflow, account_index, token_index, amount, l1_tx_hash, nonce));

        pub_key = get_public_key("4");
        origin = 4u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit4_balance_overflow: [u8; 64] = get_sign("deposit4_balance_overflow");
        account_index = 4u32;
        amount = U256::from(10);
        l1_tx_hash = U256::from(8);
        nonce = 5u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit4_balance_overflow, account_index, token_index, amount, l1_tx_hash, nonce));

        // will return 1 to balance of 3 leading to balance overflow
        origin = 4u64;
        let sign_bid_nft_has_bidder: [u8; 64] = get_sign("bid_nft_has_bidder_balance_overflow");
        amount = U256::from(2);
        nonce = 1u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft_has_bidder, nft_id, amount, nonce), Error::<Test>::BalanceOverflow);
    })
}

#[test]
fn bid_nft_balance_not_enough() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));


        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft_balance_not_enough: [u8; 64] = get_sign("bid_nft_balance_not_enough");
        // balance of 3 is 10, so it is not enough
        amount = U256::from(11);
        nonce = 1u64;
        assert_noop!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft_balance_not_enough, nft_id, amount, nonce), Error::<Test>::BalanceNotEnough);
    })
}

#[test]
// change owner of nft from 2 to the bidder 3
fn finalize_nft_works() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 2u64;
        let sign_finalize_nft: [u8; 64] = get_sign("finalize_nft");
        nonce = 1u64;
        assert_ok!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce));

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
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        // not set_key to accountIndex 4
        origin = 4u64;
        let sign_finalize_nft: [u8; 64] = get_sign("finalize_nft");
        nonce = 0u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn finalize_nft_invalid_nftindex() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let mut nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        // there is no bidder, so the nft_id 4u32 is invalid
        origin = 2u64;
        let sign_finalize_nft_no_bidder: [u8; 64] = get_sign("finalize_nft_no_bidder");
        nonce = 1u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft_no_bidder, nft_id, nonce), Error::<Test>::InvalidNFTIndex);

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 2u64;
        let sign_finalize_nft: [u8; 64] = get_sign("finalize_nft");
        // nft_id exceeds the range 20 bits
        nft_id = 1u32 << 20;
        nonce = 1u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce), Error::<Test>::InvalidNFTIndex);

        // nft_id should not be zero
        nft_id = 0;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce), Error::<Test>::InvalidNFTIndex);
    })
}

#[test]
fn finalize_nft_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 2u64;
        let sign_finalize_nft: [u8; 64] = get_sign("finalize_nft");
        // the true nonce is 1u64, set nonce to 2u64 to verify NonceInconsistent error
        nonce = 2u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn finalize_nft_invalid_signature() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 2u64;
        let mut sign_finalize_nft: [u8; 64] = get_sign("finalize_nft");
        // set sign_finalize_nft[0] to zero to verify InvalidSignature error
        sign_finalize_nft[0] = 0;
        nonce = 1u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn finalize_nft_is_not_owner() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        // the owner is 2, 1u64 is not owner
        origin = 1u64;
        let sign_finalize_nft_is_not_owner: [u8; 64] = get_sign("finalize_nft_is_not_owner");
        nonce = 4u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft_is_not_owner, nft_id, nonce), Error::<Test>::IsNotOwner);
    })
}

#[test]
fn finalize_nft_balance_overflow() {
    new_test_ext().execute_with(|| {
        let mut pub_key: [u8; 32] = get_public_key("0");
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("1");
        origin = 1u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        pub_key = get_public_key("2");
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit2: [u8; 64] = get_sign("deposit2");
        let mut account_index = 2u32;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(13);
        let mut nonce = 1u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let sign_deposit_nft: [u8; 64] = get_sign("deposit_nft");
        let nft_id = 4u32;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        pub_key = get_public_key("3");
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        let sign_deposit3: [u8; 64] = get_sign("deposit3");
        account_index = 3u32;
        l1_tx_hash = U256::from(12);
        nonce = 3u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit3, account_index, token_index, amount, l1_tx_hash, nonce));

        origin = 3u64;
        let sign_bid_nft: [u8; 64] = get_sign("bid_nft");
        amount = U256::from(1);
        nonce = 1u64;
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        origin = 1u64;
        let sign_deposit2_balance_overflow: [u8; 64] = get_sign("deposit2_finalize_balance_overflow");
        account_index = 2u32;
        amount = (U256::from(1) << 250) - 11;
        l1_tx_hash = U256::from(2);
        nonce = 4u64;
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2_balance_overflow, account_index, token_index, amount, l1_tx_hash, nonce));

        // balance of accountIndex 2 is (U256::from(1) << 250) - 1, if finalize_nft is executed, balance will overflow
        origin = 2u64;
        let sign_finalize_nft: [u8; 64] = get_sign("finalize_nft");
        nonce = 1u64;
        assert_noop!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce), Error::<Test>::BalanceOverflow);
    })
}
