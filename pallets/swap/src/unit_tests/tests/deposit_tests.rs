use super::*;

#[test]
fn deposit_works() {
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
        //token_index is set to 1u32 temporiarily
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        assert_eq!(BalanceMap::get((&account_index, token_index)), amount);
    })
}

#[test]
fn deposit_noaccess() {
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

        //Deposit 10 into accountIndex 2, caller is accountIndex 0
        //AccoutIndex 0 is not admin
        origin = 0u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_0);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::NoAccess);
    })
}

#[test]
fn deposit_account_not_exists() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let mut origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //Deposit 10 into accountIndex 2, caller is accountIndex 1
        //Not setKey for accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        //Only setKey for accountIndex 0, so use secret_key_0 here
        let command_sign = BabyJubjub::sign(&command, &secret_key_0);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn deposit_invalid_tokenindex() {
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
        //1u32 << 10 exceeds the range of token_index
        let token_index = 1u32 << 10;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidTokenIndex);
    })
}

#[test]
fn deposit_invalid_account() {
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

        //SetKey for accountIndex 3
        let mut origin = 3u64;
        let secret_key_3 = [5u8; 32];
        let pub_key_3 = BabyJubjub::pubkey_from_secretkey(&secret_key_3).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //Deposit 10 into accountIndex 3, caller is accountIndex 1
        //Not setKey for accountIndex 2
        origin = 1u64;
        let account_index = 3u32;
        let token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidAccount);
    })
}

#[test]
fn deposit_l1tx_exists() {
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
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //Deposit with same arguments
        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::L1TXExists);
    })
}

#[test]
fn deposit_invalid_amount() {
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

        //Deposit U256::from(10) << 250 into accountIndex 2, caller is accountIndex 1
        origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        //U256::from(10) << 250 exceeds the range of amount
        let amount = U256::from(10) << 250;
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidAmount);
    })
}

#[test]
fn deposit_nonce_inconsistent() {
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
        let l1_tx_hash = U256::from(0);
        //true nonce is 1u64
        let nonce = 2u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn deposit_invalid_signature() {
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
        let mut token_index = 1u32;
        let amount = U256::from(10);
        let l1_tx_hash = U256::from(0);
        let nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        //command_sign_formatted use token_index 1u32
        token_index = 0u32;

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn deposit_balance_overflow() {
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
        let mut l1_tx_hash = U256::from(0);
        let mut nonce = 1u64;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce));

        //Deposit (U256::from(1) << 250) - 10 into accountIndex 2, caller is accountIndex 1
        //After deposit, the balance of accountIndex 2 exceeds the range of balance
        amount = (U256::from(1) << 250) - 10;
        l1_tx_hash = U256::from(1);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::deposit(Origin::signed(origin), command_sign_formatted, account_index, token_index, amount, l1_tx_hash, nonce), Error::<Test>::BalanceOverflow);
    })
}
