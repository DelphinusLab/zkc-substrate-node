use super::*;
use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};

#[path = "unit_tests/tests/set_key_tests.rs"]
mod set_key_tests;

#[path = "unit_tests/tests/deposit_tests.rs"]
mod deposit_tests;

#[path = "unit_tests/tests/deposit_nft_tests.rs"]
mod deposit_nft_tests;

#[path = "unit_tests/tests/withdraw_nft_tests.rs"]
mod withdraw_nft_tests;

#[path = "unit_tests/tests/transfer_nft_tests.rs"]
mod transfer_nft_tests;

#[path = "unit_tests/tests/bid_nft_tests.rs"]
mod bid_nft_tests;

#[path = "unit_tests/tests/finalize_nft_tests.rs"]
mod finalize_nft_tests;

#[path = "unit_tests/tests/add_pool_tests.rs"]
mod add_pool_tests;

#[path = "unit_tests/tests/pool_supply_tests.rs"]
mod pool_supply_tests;

#[path = "unit_tests/tests/pool_retrieve_tests.rs"]
mod pool_retrieve_tests;

#[path = "unit_tests/tests/swap_tests.rs"]
mod swap_tests;

#[path = "unit_tests/tests/get_share_change_tests.rs"]
mod get_share_change_tests;

#[path = "unit_tests/tests/calculate_new_k_tests.rs"]
mod calculate_new_k_tests;

#[path = "unit_tests/tests/calculate_swap_result_amount_tests.rs"]
mod calculate_swap_result_amount_tests;
