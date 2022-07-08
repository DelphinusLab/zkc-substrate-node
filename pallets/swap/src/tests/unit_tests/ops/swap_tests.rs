use super::*;

fn prepare_unit_test() {
    //SetKey for accountIndex 0
    let mut origin = 0u64;
    let pub_key_0: [u8; 32] = [
        31, 191,  89, 175,  20, 249,  30,  36,
        241, 189, 202, 124,  86, 229, 209, 121,
        66, 200, 153,  22, 214,  74, 245, 240,
        154,  86, 172,  63, 104, 123, 204,   6
    ];
    assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

    //SetKey for accountIndex 1
    origin = 1u64;
    let pub_key_1: [u8; 32] = [
        87, 18,  13,  76, 122, 234,  36, 117,
        25, 95, 106, 155, 114, 225, 157, 106,
        60, 78, 106, 209,  86, 159, 227,  49,
        150, 88,   7,  37, 132,   7, 145,  28
    ];
    assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_1));

    //SetKey for accountIndex 2
    origin = 2u64;
    let pub_key_2: [u8; 32] = [
        46, 138,  20, 177,   1, 234,   6,  19,
        31,   3, 154, 170, 114, 243,  92, 197,
        134, 178, 215, 240, 105,  43,  82, 152,
        211,  56, 225, 138, 211,  60, 184,  11
    ];
    assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_2));

    //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
    origin = 1u64;
    let token_index_0 = 0u32;
    let token_index_1 = 1u32;
    let nonce = 1u64;
    let secret_key_1 = [
        143, 209,  13,  17, 171, 232,  44, 222,
        13, 243, 179, 199, 195, 184,  29,   4,
        200,  51,  13,  16,  39, 124, 194, 125,
        49, 180, 255,  97, 249,  95,   1, 203
    ];

    let mut command = [0u8; 81];
    command[0] = OP_ADDPOOL;
    command[1..9].copy_from_slice(&nonce.to_be_bytes());
    command[9..13].copy_from_slice(&token_index_0.to_be_bytes());
    command[13..17].copy_from_slice(&token_index_1.to_be_bytes());

    let command_sign = BabyJubjub::sign(&command, &secret_key_1);
    let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
    command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
    command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

    assert_ok!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce));
 
    //Deposit 2500 into accountIndex 2, caller is accountIndex 1
    origin = 1u64;
    let account_index = 2u32;
    let mut token_index = 0u32;
    let amount = U256::from(2500);
    let mut l1_tx_hash = U256::from(0);
    let mut nonce = 2u64;

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

    //Deposit 2500 into accountIndex 2, caller is accountIndex 1
    token_index = 1u32;
    l1_tx_hash = U256::from(1);
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

    //PoolSupply amount0 1000 and amount1 1000 for poolIndex 0, caller is accountIndex 2
    let origin = 2u64;
    let account_index = 2u32;
    let pool_index = 0u32;
    let amount0 = U256::from(1000);
    let amount1 = U256::from(1000);
    let nonce = 1u64;
    let secret_key_2 = [
        210, 199, 164, 130,  20, 202,  75,  82,
        215,  24,   9, 195,  86, 213, 230,  20,
        159, 219, 169, 225,  93, 193, 109, 240,
        185, 222, 254,  50, 115,  63,  97, 179
    ];

    let mut command = [0u8; 81];
    command[0] = OP_SUPPLY;
    command[1..9].copy_from_slice(&nonce.to_be_bytes());
    command[9..13].copy_from_slice(&account_index.to_be_bytes());
    command[13..17].copy_from_slice(&pool_index.to_be_bytes());
    command[17..49].copy_from_slice(&amount0.to_be_bytes());
    command[49..81].copy_from_slice(&amount1.to_be_bytes());

    let command_sign = BabyJubjub::sign(&command, &secret_key_2);
    let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
    command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
    command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

    assert_ok!(SwapModule::pool_supply(Origin::signed(origin), command_sign_formatted, pool_index, amount0, amount1, nonce));
}

#[test]
fn swap_works_reverse_is_zero() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2, reverse is 0
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        let amount = U256::from(100);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce));

        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, U256::from(1100), U256::from(910), U256::from(1_000_000_000_000_000_000u128)));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), U256::from(1400));

        assert_eq!(BalanceMap::get((&account_index, 1u32)), U256::from(1590));

        assert_eq!(NonceMap::<Test>::get(2u64), 3u64);
    })
}

#[test]
fn swap_works_reverse_is_one() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2, reverse is 1
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 1u8;
        let amount = U256::from(100);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];

        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce));

        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, U256::from(910), U256::from(1100), U256::from(1_000_000_000_000_000_000u128)));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), U256::from(1590));

        assert_eq!(BalanceMap::get((&account_index, 1u32)), U256::from(1400));

        assert_eq!(NonceMap::<Test>::get(2u64), 3u64);
    })
}

#[test]
fn swap_multiple_times_works() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //Swap 3 times, amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        let swap_times = 3;
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        let amount = U256::from(100);
        let mut nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];

        let (_, _, _, _, total_share) = PoolMap::get(pool_index).unwrap();
        let mut command = [0u8; 81];
        let mut index = 0;
        let mut command_sign;
        let mut command_sign_formatted :[u8; 64];
		let mut liq0 = U256::from(1000);
		let mut liq1 = U256::from(1000);
		let mut balance0 = U256::from(1500);
		let mut balance1 = U256::from(1500);
		let mut dividend: Amount;
		let mut divisor;
		let mut result_amount;

        while index < swap_times {
            command[0] = OP_SWAP;
            command[1..9].copy_from_slice(&nonce.to_be_bytes());
            command[9..13].copy_from_slice(&account_index.to_be_bytes());
            command[13..17].copy_from_slice(&pool_index.to_be_bytes());
            command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
            command[49..81].copy_from_slice(&amount.to_be_bytes());

            command_sign = BabyJubjub::sign(&command, &secret_key_2);
            command_sign_formatted = [0 as u8;64];
            command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
            command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

            assert_ok!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce));

            dividend = liq1 * amount * 1021;
            divisor = (liq0 + amount) * 1024;
            result_amount = dividend.checked_div(divisor).unwrap();

            nonce += 1;
            index += 1;
            liq0 = liq0 + amount;
            liq1 = liq1 - result_amount;
            balance0 = balance0 - amount;
            balance1 = balance1 + result_amount;
        }

        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, liq0, liq1, total_share));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), balance0);

        assert_eq!(BalanceMap::get((&account_index, 1u32)), balance1);

        assert_eq!(NonceMap::<Test>::get(2u64), nonce);
    })
}

#[test]
fn swap_account_not_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 3
        //Not setKey for accountIndex 3
        let origin = 3u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        let amount = U256::from(100);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        //There is no secret_key_3, so use secret_key_2 here
        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn swap_invalid_amount_amount_is_zero() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Swap amount 0 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        //amount should not be zero
        let amount = U256::from(0);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::InvalidAmount);
    })
}

#[test]
fn swap_invalid_amount_amount_exceeds_range() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //Swap amount U256::from(1) << 99 + 1 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        //amount exceeds the range 99 bits
        let amount = U256::from(1) << 99;
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];

        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted: [u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::InvalidAmount);
    })
}

#[test]
fn swap_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        let amount = U256::from(100);
        // True nonce is 2
        let nonce = 3u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn swap_pool_not_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 1, caller is accountIndex 2
        let origin = 2u64;
        let account_index = 2u32;
        //Not AddPool for poolIndex 1
        let pool_index = 1u32;
        let reverse = 0u8;
        let amount = U256::from(100);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::PoolNotExists);
    })
}

#[test]
fn swap_invalid_signature() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        let mut amount = U256::from(100);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());
        
        //command_sign_formatted use amount 100
        amount = U256::from(101); 

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn swap_balance_not_enough() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Swap amount 1501 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        let origin = 2u64;
        let account_index = 2u32;
        let pool_index = 0u32;
        let reverse = 0u8;
        //tokenIndex0/tokenIndex1 balance is 1500
        let amount = U256::from(1501);
        let nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_2);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::BalanceNotEnough);
    })
}

#[test]
fn swap_balance_overflow() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //Deposit (U256::from(1) << 250) - 1501 into accountIndex 2, caller is accountIndex 1
        let origin = 1u64;
        let account_index = 2u32;
        let token_index = 1u32;
        let mut amount = (U256::from(1) << 250) - 1501;
        let l1_tx_hash = U256::from(2);
        let mut nonce = 4u64;
        let secret_key_1 = [
            143, 209,  13,  17, 171, 232,  44, 222,
            13, 243, 179, 199, 195, 184,  29,   4,
            200,  51,  13,  16,  39, 124, 194, 125,
            49, 180, 255,  97, 249,  95,   1, 203
        ];

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

        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2
        //Balance is max value now
        let origin = 2u64;
        let pool_index = 0u32;
        let reverse = 0u8;
        amount = U256::from(100);
        nonce = 2u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];
        
        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());
        
        assert_noop!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce), Error::<Test>::BalanceOverflow);
    })
}
