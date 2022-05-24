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
