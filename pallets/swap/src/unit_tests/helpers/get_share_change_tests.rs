use super::*;

fn prepare_unit_test() {
    //PoolMap insert new value
    let pool_index = 0u32;
    let token_index0 = 0u32;
    let token_index1 = 1u32;
    let amount0 = U256::from(0);
    let amount1 = U256::from(0);
    let k = U256::exp10(ORDER_OF_MAGNITUDE);
    let rem = U256::from(0);
    PoolMap::insert(
        pool_index,
        (
            &token_index0.clone(),
            &token_index1.clone(),
            amount0,
            amount1,
            k,
            rem
        ),
    );
}

#[test]
fn get_share_change_works_is_supply() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //GetShareChange if is_supply is true
        let pool_index = 0u32;
        let amount = U256::from(1000);
        let is_supply = true;

        assert_ok!(get_share_change::<Test>(&pool_index, amount, is_supply));

        assert_eq!(get_share_change::<Test>(&pool_index, amount, is_supply).unwrap(), U256::from(999_999_999_999_999_999_999_999_000u128));
    })
}

#[test]
fn get_share_change_works_is_not_supply() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //GetShareChange if is_supply is false
        let pool_index = 0u32;
        let amount = U256::from(500);
        let is_supply = false;

        assert_ok!(get_share_change::<Test>(&pool_index, amount, is_supply));

        assert_eq!(get_share_change::<Test>(&pool_index, amount, is_supply).unwrap(), U256::from(500_000_000_000_000_000_000_000_000u128));
    })
}

#[test]
fn get_share_change_pool_not_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //GetShareChange if is_supply is true
        //Not AddPool for poolIndex 1
        let pool_index = 1u32;
        let amount = U256::from(1000);
        let is_supply = true;

        match get_share_change::<Test>(&pool_index, amount, is_supply) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::PoolNotExists));
            }
        }
    })
}

#[test]
fn get_share_change_invalid_amount() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //GetShareChange if is_supply is true
        let pool_index = 0u32;
        //amount exceeds the range 125 bits
        let amount = U256::from(1) << 125;
        let is_supply = true;

        match get_share_change::<Test>(&pool_index, amount, is_supply) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}
