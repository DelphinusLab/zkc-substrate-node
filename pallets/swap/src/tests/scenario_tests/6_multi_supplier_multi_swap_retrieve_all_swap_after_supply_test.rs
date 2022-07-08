use super::*;

#[test]
//A supply, B supply, multi-swap , B retrieve, A retrieve
fn multi_supplier_multi_swap_retrieve_all_works_swap_after_supply() {
    new_test_ext().execute_with(|| {
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

        //SetKey for accountIndex 3
        origin = 3u64;
        let pub_key_3: [u8; 32] = [
		  146, 117,  71, 194, 100, 217,   0, 170,
		   80,  71,  15,  69, 223, 145, 236, 240,
		   50,  22, 252, 115,  27, 101,  23, 224,
		  148,  89, 191,  27, 216, 240,  68,   3
		];
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_3));

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        origin = 1u64;
        let token_index_0 = 0u32;
        let token_index_1 = 1u32;
        let mut nonce = 1u64;
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

        let mut command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce));

        //Deposit 2000 into accountIndex 2, caller is accountIndex 1, tokenIndex is 0
        origin = 1u64;
        let mut account_index = 2u32;
        let mut token_index = 0u32;
        let mut amount = U256::from(2000);
        let mut l1_tx_hash = U256::from(0);
        nonce = 2u64;

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

        //Deposit 2000 into accountIndex 2, caller is accountIndex 1, tokenIndex is 1
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

        //Deposit 2000 into accountIndex 3, caller is accountIndex 1, tokenIndex is 0
        account_index = 3u32;
        token_index = 0u32;
        amount = U256::from(2000);
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

        //Deposit 2000 into accountIndex 3, caller is accountIndex 1, tokenIndex is 1
        token_index = 1u32;
        l1_tx_hash = U256::from(3);
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

        //Deposit 2000 into accountIndex 0, caller is accountIndex 1, tokenIndex is 0
        account_index = 0u32;
        token_index = 0u32;
        l1_tx_hash = U256::from(4);
        nonce = 6u64;

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

        //Deposit 2000 into accountIndex 0, caller is accountIndex 1, tokenIndex is 1
        token_index = 1u32;
        l1_tx_hash = U256::from(5);
        nonce = 7u64;

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
        origin = 2u64;
        account_index = 2u32;
        let pool_index = 0u32;
        let mut amount0 = U256::from(1000);
        let mut amount1 = U256::from(1000);
        nonce = 1u64;
        let secret_key_2 = [
            210, 199, 164, 130,  20, 202,  75,  82,
            215,  24,   9, 195,  86, 213, 230,  20,
            159, 219, 169, 225,  93, 193, 109, 240,
            185, 222, 254,  50, 115,  63,  97, 179
        ];

        command = [0u8; 81];
        command[0] = OP_SUPPLY;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount0.to_be_bytes());
        command[49..81].copy_from_slice(&amount1.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::pool_supply(Origin::signed(origin), command_sign_formatted, pool_index, amount0, amount1, nonce));

        //PoolSupply amount0 1000 and amount1 1000 for poolIndex 0, caller is accountIndex 3
        origin = 3u64;
        account_index = 3u32;
        amount0 = U256::from(1000);
        amount1 = U256::from(1000);
        nonce = 1u64;
        let secret_key_3 = [
            8, 123, 229, 206, 38, 143,  38, 167,
            240, 190,  94,  53, 94, 181, 177, 226,
            58,  65, 254,  37, 26, 200, 197, 220,
            204, 202, 148, 243, 45, 145,   9,   7
        ];

        command = [0u8; 81];
        command[0] = OP_SUPPLY;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount0.to_be_bytes());
        command[49..81].copy_from_slice(&amount1.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::pool_supply(Origin::signed(origin), command_sign_formatted, pool_index, amount0, amount1, nonce));

        //Swap amount 100 from tokenIndex0 to tokenIndex1 for poolIndex 0, caller is accountIndex 0, reverse is 0
        //Swap afer all supply
        origin = 0u64;
        account_index = 0u32;
        let mut reverse = 0u8;
        amount = U256::from(100);
        nonce = 1u64;
        let secret_key_0 = [
            227, 102, 100, 225, 229,  10,  36,  64,
            122, 107, 115, 225, 109, 250, 167, 226,
            127, 193,  60, 208,  74,  89, 100,  44,
            140, 130,  52, 195,  95, 192,  40,  50
        ];

        command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_0);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce));

        //Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 0, reverse is 1
        reverse = 1u8;
        amount = U256::from(100);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_0);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::swap(Origin::signed(origin), command_sign_formatted, pool_index, reverse, amount, nonce));

        //PoolRetrieve amount0 998 and amount1 1003 for poolIndex 0, caller is accountIndex 3
        origin = 3u64;
        account_index = 3u32;
        amount0 = U256::from(998);
        amount1 = U256::from(1003);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_RETRIEVE;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount0.to_be_bytes());
        command[49..81].copy_from_slice(&amount1.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_3);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::pool_retrieve(Origin::signed(origin), command_sign_formatted, pool_index, amount0, amount1, nonce));

        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, U256::from(998), U256::from(1003), U256::from(1_000_000_000_000_000_000u128)));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), U256::from(1998));

        assert_eq!(BalanceMap::get((&account_index, 1u32)), U256::from(2003));

        assert_eq!(ShareMap::get((&account_index, &pool_index)), U256::from(0));

        assert_eq!(NonceMap::<Test>::get(3u64), 3u64);

        //PoolRetrieve amount0 998 and amount1 1003 for poolIndex 0, caller is accountIndex 2
        origin = 2u64;
        account_index = 2u32;
        amount0 = U256::from(998);
        amount1 = U256::from(1003);
        nonce = 2u64;

        command = [0u8; 81];
        command[0] = OP_RETRIEVE;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount0.to_be_bytes());
        command[49..81].copy_from_slice(&amount1.to_be_bytes());

        command_sign = BabyJubjub::sign(&command, &secret_key_2);
        command_sign_formatted = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::pool_retrieve(Origin::signed(origin), command_sign_formatted, pool_index, amount0, amount1, nonce));

        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, U256::from(0), U256::from(0), U256::from(0)));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), U256::from(1998));

        assert_eq!(BalanceMap::get((&account_index, 1u32)), U256::from(2003));

        assert_eq!(ShareMap::get((&account_index, &pool_index)), U256::from(0));

        assert_eq!(NonceMap::<Test>::get(2u64), 3u64);
    })
}
