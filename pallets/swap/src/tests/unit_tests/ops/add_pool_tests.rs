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
}

#[test]
fn add_pool_works() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
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

        assert_eq!(PoolIndexMap::get((&token_index_0, &token_index_1)).unwrap(), 0u32);

        assert_eq!(PoolMap::get(0u32).unwrap(), (0u32, 1u32, U256::from(0), U256::from(0), U256::from(0)));

        assert_eq!(NonceMap::<Test>::get(1u64), 2);
    })
}

#[test]
fn add_pool_noaccess() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 0
        //Caller 0 is not admin
        let origin = 0u64;
        let token_index_0 = 0u32;
        let token_index_1 = 1u32;
        let nonce = 1u64;
        let secret_key_0 = [
            227, 102, 100, 225, 229,  10,  36,  64,
            122, 107, 115, 225, 109, 250, 167, 226,
            127, 193,  60, 208,  74,  89, 100,  44,
            140, 130,  52, 195,  95, 192,  40,  50
        ];

        let mut command = [0u8; 81];
        command[0] = OP_ADDPOOL;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&token_index_0.to_be_bytes());
        command[13..17].copy_from_slice(&token_index_1.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_0);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::NoAccess);
    })
}

#[test]
fn add_pool_invalid_tokenindex_tokenindex_0_exceeds_range() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 1u32 << 10 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
        //1u32 << 10 exceeds the range 10bits
        let token_index_0 = 1u32 << 10;
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

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::InvalidTokenIndex);
    })
}

#[test]
fn add_pool_invalid_tokenindex_tokenindex_1_exceeds_range() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 1 and tokenIndex1 1u32 << 10 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
        let token_index_0 = 1u32;
        //1u32 << 10 exceeds the range 10bits
        let token_index_1 = 1u32 << 10;
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

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::InvalidTokenIndex);
    })
}

#[test]
fn add_pool_invalid_tokenpair() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 1 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
        //tokenIndex0 != tokenIndex1
        let token_index_0 = 1u32;
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

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::InvalidTokenPair);
    })
}

#[test]
fn add_pool_account_not_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 2
        //Not setKey for accountIndex 2
        let origin = 2u64;
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

        //There is no secret_key_2, so use secret_key_1 here
        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::AccountNotExists);
    })
}

#[test]
fn add_pool_nonce_inconsistent() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
        let token_index_0 = 0u32;
        let token_index_1 = 1u32;
        //True nonce is 1
        let nonce = 2u64;
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

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::NonceInconsistent);
    })
}

#[test]
fn add_pool_invalid_signature() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
        let mut token_index_0 = 0u32;
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

        //command_sign_formatted use token_index_0 0u32
        token_index_0 = 2u32;

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::InvalidSignature);
    })
}

#[test]
fn add_pool_pool_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        let origin = 1u64;
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

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_ok!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce));

        //AddPool tokenIndex0 0 and tokenIndex1 1 for poolIndex 0, caller is accountIndex 1
        //AddPool twice with same tokenIndex0 and tokenIndex1
        nonce = 2u64;

        let mut command = [0u8; 81];
        command[0] = OP_ADDPOOL;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&token_index_0.to_be_bytes());
        command[13..17].copy_from_slice(&token_index_1.to_be_bytes());

        let command_sign = BabyJubjub::sign(&command, &secret_key_1);
        let mut command_sign_formatted :[u8; 64] = [0 as u8;64];
        command_sign_formatted[..32].copy_from_slice(&command_sign.r.encode());
        command_sign_formatted[32..].copy_from_slice(&command_sign.s.encode());

        assert_noop!(SwapModule::add_pool(Origin::signed(origin), command_sign_formatted, token_index_0, token_index_1, nonce), Error::<Test>::PoolExists);
    })
}
