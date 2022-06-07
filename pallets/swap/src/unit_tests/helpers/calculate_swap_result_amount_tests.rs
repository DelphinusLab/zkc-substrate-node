use super::*;

#[test]
fn calculate_swap_result_amount_works() {
    new_test_ext().execute_with(|| {
        //CalculateSwapResultAmount
        let amount_input = U256::from(1000);
        let amount_output = U256::from(1000);
        let amount = U256::from(500);

        assert_ok!(calculate_swap_result_amount::<Test>(amount_input, amount_output, amount));

        assert_eq!(calculate_swap_result_amount::<Test>(amount_input, amount_output, amount).unwrap(), U256::from(332));
    })
}

#[test]
fn calculate_swap_result_amount_invalid_amount_amount_input_exceeds_range() {
    new_test_ext().execute_with(|| {
        //CalculateSwapResultAmount
        //amount_input exceeds the range 125bits
        let amount_input = U256::from(1) << 125;
        let amount_output = U256::from(1000);
        let amount = U256::from(500);

        match calculate_swap_result_amount::<Test>(amount_input, amount_output, amount) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}

#[test]
fn calculate_swap_result_amount_invalid_amount_amount_input_is_zero() {
    new_test_ext().execute_with(|| {
        //CalculateSwapResultAmount
        //amount_input should not be zero
        let amount_input = U256::from(0);
        let amount_output = U256::from(1000);
        let amount = U256::from(500);

        match calculate_swap_result_amount::<Test>(amount_input, amount_output, amount) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}

#[test]
fn calculate_swap_result_amount_invalid_amount_amount_output_exceeds_range() {
    new_test_ext().execute_with(|| {
        //CalculateSwapResultAmount
        let amount_input = U256::from(0);
        //amount_output exceeds the range 125bits
        let amount_output = U256::from(1) << 125;
        let amount = U256::from(500);

        match calculate_swap_result_amount::<Test>(amount_input, amount_output, amount) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}

#[test]
fn calculate_swap_result_amount_invalid_amount_amount_exceeds_range() {
    new_test_ext().execute_with(|| {
        //CalculateSwapResultAmount
        let amount_input = U256::from(1000);
        let amount_output = U256::from(1000);
        //amount exceeds the range 125 bits
        let amount = U256::from(1) << 125;

        match calculate_swap_result_amount::<Test>(amount_input, amount_output, amount) {
            Ok(_) => assert!(false),
            Err(e) => {
                assert!(matches!(e, Error::<Test>::InvalidAmount));
            }
        }
    })
}
