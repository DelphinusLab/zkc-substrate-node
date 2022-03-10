use super::*;
use crate::{mock::*};
use frame_support::{assert_ok};
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::collections::HashMap;

const OP_DEPOSIT: u8 = 0u8;
const OP_DEPOSIT_NFT: u8 = 7u8;
const OP_WITHDRAW_NFT: u8 = 8u8;
const OP_TRANSFER_NFT: u8 = 9u8;
const OP_BID_NFT: u8 = 10u8;
const OP_FINALIZE_NFT: u8 = 11u8;

#[test]
fn nft_works() {
	new_test_ext().execute_with(|| {
        let path = "./config/pk.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let key: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut pubKey: [u8; 32] = [0; 32];
        let mut count = 0;
        for (_key, value) in key {
            pubKey[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::set_key(Origin::signed(1), pubKey));

        let token_index = 1u32;
        let mut account_index = 1u32;
        let mut amount = U256::from(10u8);
        let mut l1_tx_hash = U256::from(15u8);
        let mut nonce = 1u64;
        let mut command = [0u8; 81];
        let path = "./config/deposit1.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_deposit1_64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_deposit1: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_deposit1_64 {
            sign_deposit1[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::deposit(Origin::signed(1), sign_deposit1, account_index, token_index, amount, l1_tx_hash, nonce));

        let nft_id = 6u32;
        l1_tx_hash = U256::from(16u8);
        nonce = 2u64;
        command = [0u8; 81];
        command[0] = OP_DEPOSIT_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1_tx_hash.to_be_bytes());
        let path = "./config/deposit_nft.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_deposit_nft64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_deposit_nft: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_deposit_nft64 {
            sign_deposit_nft[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::deposit_nft(Origin::signed(1), sign_deposit_nft, account_index, nft_id, l1_tx_hash, nonce));

        assert_ok!(SwapModule::set_key(Origin::signed(2), pubKey));

        account_index = 2u32;
        amount = U256::from(10u8);
        l1_tx_hash = U256::from(12u8);
        nonce = 1u64;
        command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());
        command[49..81].copy_from_slice(&l1_tx_hash.to_be_bytes());
        let path = "./config/deposit2.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_deposit2_64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_deposit2: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_deposit2_64 {
            sign_deposit2[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::deposit(Origin::signed(2), sign_deposit2, account_index, token_index, amount, l1_tx_hash, nonce));

        let owner = 1u32;
        amount = U256::from(1u8);
        nonce = 2u64;
        command[0] = OP_BID_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&owner.to_be_bytes());
        command[13..17].copy_from_slice(&account_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());
        command[49..53].copy_from_slice(&nft_id.to_be_bytes());
        let path = "./config/bid_nft.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_bid_nft64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_bid_nft: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_bid_nft64 {
            sign_bid_nft[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::bid_nft(Origin::signed(2), sign_bid_nft, nft_id, owner, amount, nonce));

        assert_ok!(SwapModule::set_key(Origin::signed(5), pubKey));

        account_index = 1u32;
        let recipent = 5u32;
        nonce = 3u64;
        command[0] = OP_TRANSFER_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&recipent.to_be_bytes());
        command[17..21].copy_from_slice(&nft_id.to_be_bytes());
        let path = "./config/transfer.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_transfer_nft64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_transfer_nft: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_transfer_nft64 {
            sign_transfer_nft[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::transfer_nft(Origin::signed(1), sign_transfer_nft, nft_id, owner, nonce));

        account_index = 5u32;
        let bidder = 2u32;
        amount = U256::from(10u8);
        nonce = 1u64;
        command[0] = OP_FINALIZE_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&bidder.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());
        command[49..53].copy_from_slice(&nft_id.to_be_bytes());
        let path = "./config/finalize_nft.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_finalize_nft64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_finalize_nft: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_finalize_nft64 {
            sign_finalize_nft[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::finalize_nft(Origin::signed(5), sign_finalize_nft, nft_id, bidder, amount, nonce));

        account_index = 2u32;
        let l1account = U256::from(3u8);
        nonce = 3u64;
        command[0] = OP_WITHDRAW_NFT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&nft_id.to_be_bytes());
        command[17..49].copy_from_slice(&l1account.to_be_bytes());
        let path = "./config/withdraw_nft.json";
        let file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                println!("open failed: {}", e);
                process::exit(1);
            }
        };
        let reader = BufReader::new(file);
        let sign_withdraw_nft64: HashMap<String, u8> = match serde_json::from_reader(reader) {
            Ok(c) => c,
            Err(e) => {
                println!("Read file error: {}", e);
                process::exit(1);
            }
        };
        let mut sign_withdraw_nft: [u8; 64] = [0; 64];
        count = 0;
        for (_key, value) in sign_withdraw_nft64 {
            sign_withdraw_nft[count] = value;
            count += 1;
        }
        assert_ok!(SwapModule::withdraw_nft(Origin::signed(2), sign_withdraw_nft, nft_id, l1account, nonce));
    })
}
