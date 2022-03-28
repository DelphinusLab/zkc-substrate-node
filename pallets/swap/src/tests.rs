use super::*;
use crate::{mock::*};
use frame_support::{assert_ok};
use std::fs::File;
use std::io::BufReader;
use std::process;

#[test]
fn nft_works() {
	new_test_ext().execute_with(|| {
        // public key
        let mut path = "./src/unit_tests/input/pk.json";
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let mut reader = BufReader::new(file);
        let pub_key: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };

        // set key
        let mut origin = 0u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        origin = 1u64;
        // set key
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // sign_deposit1
        let mut account_index = 1u32;
        let mut nonce = 1u64;
        let token_index = 1u32;
        let mut amount = U256::from(10);
        let mut l1_tx_hash = U256::from(0);
        path = "./src/unit_tests/input/deposit1_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_deposit1_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/deposit1_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_deposit1_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_deposit1: [u8; 64] = [0; 64];
        sign_deposit1[..32].copy_from_slice(&sign_deposit1_1);
        sign_deposit1[32..].copy_from_slice(&sign_deposit1_2);
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit1, account_index, token_index, amount, l1_tx_hash, nonce));

        //sign_deposit_nft
        let nft_id = 4u32;
        nonce = 2u64;
        l1_tx_hash = U256::from(1);
        path = "./src/unit_tests/input/deposit_nft_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_deposit_nft_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/deposit_nft_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_deposit_nft_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_deposit_nft: [u8; 64] = [0; 64];
        sign_deposit_nft[..32].copy_from_slice(&sign_deposit_nft_1);
        sign_deposit_nft[32..].copy_from_slice(&sign_deposit_nft_2);
        assert_ok!(SwapModule::deposit_nft(Origin::signed(origin), sign_deposit_nft, nft_id, l1_tx_hash, nonce));

        // set key
        origin = 2u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // sign_deposit2
        account_index = 2u32;
        amount = U256::from(10);
        l1_tx_hash = U256::from(12);
        nonce = 1u64;
        path = "./src/unit_tests/input/deposit2_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_deposit2_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/deposit2_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_deposit2_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_deposit2: [u8; 64] = [0; 64];
        sign_deposit2[..32].copy_from_slice(&sign_deposit2_1);
        sign_deposit2[32..].copy_from_slice(&sign_deposit2_2);
        assert_ok!(SwapModule::deposit(Origin::signed(origin), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        // bid_nft
        nonce = 2u64;
        amount = U256::from(1);
        path = "./src/unit_tests/input/bid_nft_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_bid_nft_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/bid_nft_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_bid_nft_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_bid_nft: [u8; 64] = [0; 64];
        sign_bid_nft[..32].copy_from_slice(&sign_bid_nft_1);
        sign_bid_nft[32..].copy_from_slice(&sign_bid_nft_2);
        assert_ok!(SwapModule::bid_nft(Origin::signed(origin), sign_bid_nft, nft_id, amount, nonce));

        // set key
        origin = 3u64;
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key));

        // transfer_nft
        origin = 1u64;
        nonce = 3u64;
        let recipient = 3u32;
        path = "./src/unit_tests/input/transfer_nft_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_transfer_nft_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/transfer_nft_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_transfer_nft_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_transfer_nft: [u8; 64] = [0; 64];
        sign_transfer_nft[..32].copy_from_slice(&sign_transfer_nft_1);
        sign_transfer_nft[32..].copy_from_slice(&sign_transfer_nft_2);
        assert_ok!(SwapModule::transfer_nft(Origin::signed(origin), sign_transfer_nft, nft_id, recipient, nonce));

        // finalize_nft
        origin = 3u64;
        nonce = 1u64;
        path = "./src/unit_tests/input/finalize_nft_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_finalize_nft_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/finalize_nft_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_finalize_nft_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_finalize_nft: [u8; 64] = [0; 64];
        sign_finalize_nft[..32].copy_from_slice(&sign_finalize_nft_1);
        sign_finalize_nft[32..].copy_from_slice(&sign_finalize_nft_2);
        assert_ok!(SwapModule::finalize_nft(Origin::signed(origin), sign_finalize_nft, nft_id, nonce));

        // withdraw_nft
        origin = 2u64;
        let l1account = U256::from(3);
        nonce = 3u64;
        path = "./src/unit_tests/input/withdraw_nft_1.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_withdraw_nft_1: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        path = "./src/unit_tests/input/withdraw_nft_2.json";
        file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        reader = BufReader::new(file);
        let sign_withdraw_nft_2: [u8; 32] = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_withdraw_nft: [u8; 64] = [0; 64];
        sign_withdraw_nft[..32].copy_from_slice(&sign_withdraw_nft_1);
        sign_withdraw_nft[32..].copy_from_slice(&sign_withdraw_nft_2);
        assert_ok!(SwapModule::withdraw_nft(Origin::signed(origin), sign_withdraw_nft, nft_id, l1account, nonce));
    })
}
