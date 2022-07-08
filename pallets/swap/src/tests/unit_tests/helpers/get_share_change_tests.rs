use super::*;

fn prepare_unit_test() {
    //PoolMap insert new value
    let pool_index = 0u32;
    let token_index0 = 0u32;
    let token_index1 = 1u32;
    let amount0 = U256::from(0);
    let amount1 = U256::from(0);
    let total_share = U256::from(0);
    PoolMap::insert(
        pool_index,
        (
            &token_index0.clone(),
            &token_index1.clone(),
            amount0,
            amount1,
            total_share
        ),
    );
}

#[test]
fn get_share_change_works_is_supply_old_amount0_is_zero() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //GetShareChange if is_supply is true, original amount0 is zero
        let pool_index = 0u32;
        let amount = U256::from(1000);
        let is_supply = true;

        assert_ok!(get_share_change::<Test>(&pool_index, amount, is_supply));

        assert_eq!(get_share_change::<Test>(&pool_index, amount, is_supply).unwrap(), U256::from(1_000_000_000_000_000_000u128));
    })
}

#[test]
fn get_share_change_works_is_supply_old_amount0_is_not_zero() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1000);
        let total_share = U256::from(1000);
        PoolMap::insert(
            pool_index,
            (
                &token_index0.clone(),
                &token_index1.clone(),
                amount0,
                amount1,
                total_share
            ),
        );

        //GetShareChange if is_supply is true, original amount0 is not zero
        let amount = U256::from(1000);
        let is_supply = true;

        assert_ok!(get_share_change::<Test>(&pool_index, amount, is_supply));

        assert_eq!(get_share_change::<Test>(&pool_index, amount, is_supply).unwrap(), U256::from(1000));
    })
}

#[test]
fn get_share_change_works_is_not_supply_rem_is_zero() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1000);
        let total_share = U256::from(1000);
        PoolMap::insert(
            pool_index,
            (
                &token_index0.clone(),
                &token_index1.clone(),
                amount0,
                amount1,
                total_share
            ),
        );

        //GetShareChange if is_supply is false, amount * total_share % amount0 == 0
        let amount = U256::from(500);
        let is_supply = false;

        assert_ok!(get_share_change::<Test>(&pool_index, amount, is_supply));

        assert_eq!(get_share_change::<Test>(&pool_index, amount, is_supply).unwrap(), U256::from(500));
    })
}

#[test]
fn get_share_change_works_is_not_supply_rem_is_not_zero() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        let amount0 = U256::from(333);
        let amount1 = U256::from(333);
        let total_share = U256::from(1000);
        PoolMap::insert(
            pool_index,
            (
                &token_index0.clone(),
                &token_index1.clone(),
                amount0,
                amount1,
                total_share
            ),
        );

        //GetShareChange if is_supply is false, amount * total_share % amount0 != 0
        let amount = U256::from(500);
        let is_supply = false;

        assert_ok!(get_share_change::<Test>(&pool_index, amount, is_supply));

        assert_eq!(get_share_change::<Test>(&pool_index, amount, is_supply).unwrap(), U256::from(1502));
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
        //amount exceeds the range 99 bits
        let amount = U256::from(1) << 99;
        let is_supply = true;

        match get_share_change::<Test>(&pool_index, amount, is_supply) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}

#[test]
fn get_share_change_overflow_during_calculation() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        let amount0 = U256::from(500);
        let amount1 = U256::from(500);
        let total_share = (U256::from(1) << 250) - 1;
        PoolMap::insert(
            pool_index,
            (
                &token_index0.clone(),
                &token_index1.clone(),
                amount0,
                amount1,
                total_share
            ),
        );

        // 2 * (2^250 - 1) would overflow during calculation
        let amount = U256::from(2);
        let is_supply = false;

        match get_share_change::<Test>(&pool_index, amount, is_supply) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InternalCalcOverflow));
            }
        }
    })
}
