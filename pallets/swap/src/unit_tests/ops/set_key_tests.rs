use super::*;

#[test]
fn set_key_works() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        assert_eq!(get_account_index::<Test>(&0u64).unwrap(), 0u32);
    })
}

#[test]
fn set_key_invalid_key() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let mut pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        //Set pub_key_0[0] to 0u8 which is diffrent from the original value
        pub_key_0[0] = 0u8;
        assert_noop!(SwapModule::set_key(Origin::signed(origin), pub_key_0), Error::<Test>::InvalidKey);
    })
}

#[test]
fn set_key_account_exists() {
    new_test_ext().execute_with(|| {
        //SetKey for accountIndex 0
        let origin = 0u64;
        let secret_key_0 = [2u8; 32];
        let pub_key_0 = BabyJubjub::pubkey_from_secretkey(&secret_key_0).encode();
        assert_ok!(SwapModule::set_key(Origin::signed(origin), pub_key_0));

        //SetKey for accountIndex 0 twice
        assert_noop!(SwapModule::set_key(Origin::signed(origin), pub_key_0), Error::<Test>::AccountExists);
    })
}
