use super::*;
use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};

#[path = "tests/unit_tests/ops/set_key_tests.rs"]
mod set_key_tests;

#[path = "tests/unit_tests/ops/deposit_tests.rs"]
mod deposit_tests;

#[path = "tests/unit_tests/ops/deposit_nft_tests.rs"]
mod deposit_nft_tests;

#[path = "tests/unit_tests/ops/withdraw_nft_tests.rs"]
mod withdraw_nft_tests;

#[path = "tests/unit_tests/ops/transfer_nft_tests.rs"]
mod transfer_nft_tests;

#[path = "tests/unit_tests/ops/bid_nft_tests.rs"]
mod bid_nft_tests;

#[path = "tests/unit_tests/ops/finalize_nft_tests.rs"]
mod finalize_nft_tests;

#[path = "tests/unit_tests/ops/add_pool_tests.rs"]
mod add_pool_tests;

#[path = "tests/unit_tests/ops/pool_supply_tests.rs"]
mod pool_supply_tests;

#[path = "tests/unit_tests/ops/pool_retrieve_tests.rs"]
mod pool_retrieve_tests;

#[path = "tests/unit_tests/ops/swap_tests.rs"]
mod swap_tests;

#[path = "tests/unit_tests/helpers/get_share_change_tests.rs"]
mod get_share_change_tests;

#[path = "tests/unit_tests/helpers/calculate_swap_result_amount_tests.rs"]
mod calculate_swap_result_amount_tests;

#[path = "tests/unit_tests/helpers/calculate_amount1_to_pool_tests.rs"]
mod calculate_amount1_to_pool_tests;

#[path = "tests/scenario_tests/single_supplier_multi_swap_retrieve_all_test.rs"]
mod single_supplier_multi_swap_retrieve_all_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_cannot_retrieve_all_swap_after_supply_test.rs"]
mod multi_supplier_multi_swap_cannot_retrieve_all_swap_after_supply_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_cannot_retrieve_all_swap_not_after_supply_test.rs"]
mod multi_supplier_multi_swap_cannot_retrieve_all_swap_not_after_supply_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_retrieve_all_swap_after_supply_test.rs"]
mod multi_supplier_multi_swap_retrieve_all_swap_after_supply_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_retrieve_all_swap_not_after_supply_test.rs"]
mod multi_supplier_multi_swap_retrieve_all_swap_not_after_supply_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_multi_supply_small_amount_retrieve_all_test.rs"]
mod multi_supplier_multi_swap_multi_supply_small_amount_retrieve_all_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_multi_supply_small_amount_not_retrieve_all_test.rs"]
mod multi_supplier_multi_swap_multi_supply_small_amount_not_retrieve_all_test;

#[path = "tests/scenario_tests/multi_supplier_multi_swap_multi_retrieve_small_amount_test.rs"]
mod multi_supplier_multi_swap_multi_retrieve_small_amount_test;
