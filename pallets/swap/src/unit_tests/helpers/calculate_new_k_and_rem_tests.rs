use super::*;

fn prepare_unit_test() {
    //PoolMap insert new value
    let pool_index = 0u32;
    let token_index0 = 0u32;
    let token_index1 = 1u32;
    let amount0 = U256::from(1000);
    let amount1 = U256::from(1000);
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
fn calculate_new_k_and_rem_works_has_remainder() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //CalculateNewK if (total_old * k - rem) / total_new has remainder
        let pool_index = 0u32;
        let amount = U256::from(3);

        assert_ok!(calculate_new_k_and_rem::<Test>(&pool_index, amount));

        assert_eq!(calculate_new_k_and_rem::<Test>(&pool_index, amount).unwrap(), (U256::from(998_502_246_630_054_917_623_565u128), U256::from(695)));
    })
}

#[test]
fn calculate_new_k_and_rem_works_has_no_remainder() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //CalculateNewK if (total_old * k - rem) / total_new has no remainder
        let pool_index = 0u32;
        let amount = U256::from(2000);

        assert_ok!(calculate_new_k_and_rem::<Test>(&pool_index, amount));

        assert_eq!(calculate_new_k_and_rem::<Test>(&pool_index, amount).unwrap(), (U256::from(500_000_000_000_000_000_000_000u128), U256::from(0)));
    })
}

#[test]
fn calculate_new_k_and_rem_pool_not_exists() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //CalculateNewK if (total_old * k - rem) / total_new has no remainder
        //Not AddPool for pool_index 1
        let pool_index = 1u32;
        let amount = U256::from(3);

        match calculate_new_k_and_rem::<Test>(&pool_index, amount) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::PoolNotExists));
            }
        }
    })
}

#[test]
fn calculate_new_k_and_rem_invalid_amount() {
    new_test_ext().execute_with(|| {
        prepare_unit_test();
        
        //CalculateNewK if (total_old * k - rem) / total_new has no remainder
        let pool_index = 0u32;
        //amount exceeds the range 125 bits
        let amount = U256::from(1) << 125;

        match calculate_new_k_and_rem::<Test>(&pool_index, amount) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}
