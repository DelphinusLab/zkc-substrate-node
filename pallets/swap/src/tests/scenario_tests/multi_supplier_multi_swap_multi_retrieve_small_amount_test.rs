use super::*;

#[test]
fn multi_supplier_multi_swap_multi_retrieve_small_amount_works() {
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

        //Deposit 30000 into accountIndex 2, caller is accountIndex 1, tokenIndex is 0
        origin = 1u64;
        let mut account_index = 2u32;
        let mut token_index = 0u32;
        let mut amount = U256::from(30000);
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

        //Deposit 30000 into accountIndex 2, caller is accountIndex 1, tokenIndex is 1
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
        
        //Deposit 30000 into accountIndex 3, caller is accountIndex 1, tokenIndex is 0
        account_index = 3u32;
        token_index = 0u32;
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

        //Deposit 30000 into accountIndex 3, caller is accountIndex 1, tokenIndex is 1
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

        //PoolSupply amount0 20000 and amount1 20000 for poolIndex 0, caller is accountIndex 2
        origin = 2u64;
		account_index = 2u32;
        let pool_index = 0u32;
        let mut amount0 = U256::from(20000);
        let mut amount1 = U256::from(20000);
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

        //PoolSupply amount0 20200 and amount1 20200 for poolIndex 0, caller is accountIndex 3
        origin = 3u64;
		account_index = 3u32;
        amount0 = U256::from(20200);
        amount1 = U256::from(20200);
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

        //Swap amount 4000 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2, reverse is 0
        origin = 2u64;
        account_index = 2u32;
        let mut reverse = 0u8;
        amount = U256::from(4000);
        nonce = 2u64;

        command = [0u8; 81];
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
        
		//Swap amount 100 from tokenIndex1 to tokenIndex0 for poolIndex 0, caller is accountIndex 2, reverse is 1
        reverse = 1u8;
        amount = U256::from(100);
        nonce = 3u64;

        command = [0u8; 81];
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
        
        //PoolRetrieve amount0 21920 and amount1 18236 for poolIndex 0, caller is accountIndex 2
        amount0 = U256::from(21920u64);
        amount1 = U256::from(18236u64);
        nonce = 4u64;

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

        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, U256::from(22160), U256::from(18437), U256::from(20_209_437_386_569_872_958u128)));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), U256::from(28040));

        assert_eq!(BalanceMap::get((&account_index, 1u32)), U256::from(31763));

        assert_eq!(ShareMap::get((&account_index, &pool_index)), U256::from(9_437_386_569_872_958u128));

        assert_eq!(NonceMap::<Test>::get(2u64), 5u64);

        //PoolRetrieve 10 times, amount0 1 and amount1 0 for poolIndex 0, caller is accountIndex 2
        //Token0 belongs to accountIndex 2 is 10, accountIndex 2 should get 10
        let retrieve_times = 10;
        amount0 = U256::from(1);
        amount1 = U256::from(0);
        nonce = 5u64;

        let mut index = 0;
        command = [0u8; 81];
        
        while index < retrieve_times {
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
            
            nonce += 1;
            index += 1;
        }
 
        assert_eq!(PoolMap::get(pool_index).unwrap(), (0u32, 1u32, U256::from(22150), U256::from(18437), U256::from(20_200_317_604_355_716_878u128)));

        assert_eq!(BalanceMap::get((&account_index, 0u32)), U256::from(28050));

        assert_eq!(BalanceMap::get((&account_index, 1u32)), U256::from(31763));

        assert_eq!(ShareMap::get((&account_index, &pool_index)), U256::from(317_604_355_716_878u128));

        assert_eq!(NonceMap::<Test>::get(2u64), 15u64);
    })
}
