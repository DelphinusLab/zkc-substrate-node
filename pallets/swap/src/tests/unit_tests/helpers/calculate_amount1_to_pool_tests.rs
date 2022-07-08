use super::*;

fn prepare_unit_test() {
    //PoolMap insert new value
    let pool_index = 0u32;
    let token_index0 = 0u32;
    let token_index1 = 1u32;
    let amount0 = U256::from(2000);
    let amount1 = U256::from(3000);
    let total_share = U256::from(10000);
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
fn calculate_amount1_to_pool_works_old_amount0_is_zero() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        //Old amount0 is zero
        let old_amount0 = U256::from(0);
        let old_amount1 = U256::from(2000);
        let total_share = U256::from(10000);
        PoolMap::insert(
            pool_index,
            (
                &token_index0.clone(),
                &token_index1.clone(),
                old_amount0,
                old_amount1,
                total_share
            ),
        );
        
        //Calculate amount1_to_pool
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1500);
        let is_supply = true;

        assert_ok!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply));

        assert_eq!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply).unwrap(), U256::from(1500));
    })
}

#[test]
fn calculate_amount1_to_pool_works_old_amount0_is_not_zero_is_supply_rem_is_zero() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //Calculate amount1_to_pool
        //is_supply is true, amount0 * old amount1 % old amount0 == 0
        let pool_index = 0u32;
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1500);
        let is_supply = true;

        assert_ok!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply));

        assert_eq!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply).unwrap(), U256::from(1500));
    })
}

#[test]
fn calculate_amount1_to_pool_works_old_amount0_is_not_zero_is_supply_rem_is_not_zero() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        let amount0 = U256::from(2000);
        let amount1 = U256::from(3041);
        let total_share = U256::from(10000);
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

        //Calculate amount1_to_pool
        //is_supply is true, amount0 * old amount1 % old amount0 == 1000 
        let pool_index = 0u32;
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1600);
        let is_supply = true;

        assert_ok!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply));

        assert_eq!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply).unwrap(), U256::from(1521));
    })
}

#[test]
fn calculate_amount1_to_pool_works_old_amount0_is_not_zero_is_not_supply() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //Calculate amount1_to_pool
        let pool_index = 0u32;
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1500);
        let is_supply = false;

        assert_ok!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply));

        assert_eq!(calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply).unwrap(), U256::from(1500));
    })
}

#[test]
fn calculate_amount1_to_pool_pool_not_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();

        //PoolMap::get(&1u32) is not exist
        let pool_index = 1u32;
        let amount0 = U256::from(1000);
        let amount1 = U256::from(1500);
        let is_supply = true;

        match calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::PoolNotExists));
            }
        }

    })
}

#[test]
fn calculate_amount1_to_pool_internal_calc_overflow() {
    new_test_ext().execute_with(|| {
        //PoolMap insert new value
        let pool_index = 0u32;
        let token_index0 = 0u32;
        let token_index1 = 1u32;
        let amount0 = U256::from(3000);
        let amount1 = (U256::from(1) << 250) - 1;
        let total_share = U256::from(10000);
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

        //Calculate amount1_to_pool
        let pool_index = 0u32;
        // amount0 * old amount1 exceeds the range 250 bits 
        let amount0 = U256::from(633_825_300_114_114_700_748_351_602_687u128);
        let amount1 = U256::from(1500);
        let is_supply = true;
        
        match calculate_amount1_to_pool::<Test>(&pool_index, amount0, amount1, is_supply) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InternalCalcOverflow));
            }
        }
    })
}
