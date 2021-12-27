#![cfg(feature = "runtime-benchmarks")]

use crate::Module as Swap;
use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{boxed::*, vec, vec::*};

benchmarks! {
    charge_benchmark {
        let caller: T::AccountId = whitelisted_caller();
        let reward = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::minimum_balance();
    }: charge(RawOrigin::Signed(caller), caller.clone(), reward)

    set_key_benchmark {
        let caller: T::AccountId = whitelisted_caller();
        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);
    }: set_key(RawOrigin::Signed(caller), pubkey)

    add_pool_benchmark {
        let nonce = 0u64;
        let token_index_0 = 10u32;
        let token_index_1 = 20u32;

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = <T as Config>::ADMIN1::get();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
        Swap::<T>::set_key(caller_origin.clone(), pubkey)?;
        let nonce = nonce + 1;

        let mut command = [0u8; 81];
        command[0] = OP_ADDPOOL;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&token_index_0.to_be_bytes());
        command[13..17].copy_from_slice(&token_index_1.to_be_bytes());

        let signature = BabyJubjub::sign(&command, &key);
        let mut sign = [0u8; 64];
        sign.copy_from_slice(&[signature.r.encode(), signature.s.encode()].concat());
    }: add_pool(RawOrigin::Signed(caller), sign, token_index_0, token_index_1, nonce)

    deposit_benchmark {
        let account_index = 0u32;
        let token_index = 20u32;
        let amount = U256::from(1);
        let l1_tx_hash = U256::from(1);
        let nonce = 0u64;

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = <T as Config>::ADMIN1::get();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
        Swap::<T>::set_key(caller_origin.clone(), pubkey)?;
        let nonce = nonce + 1;

        let mut command = [0u8; 81];
        command[0] = OP_DEPOSIT;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());

        let signature = BabyJubjub::sign(&command, &key);
        let mut sign = [0u8; 64];
        sign.copy_from_slice(&[signature.r.encode(), signature.s.encode()].concat());
    }: deposit(RawOrigin::Signed(caller), sign, account_index, token_index, amount, l1_tx_hash, nonce)

    withdraw_benchmark {
        let token_index = 20u32;
        let amount = U256::from(1);
        let l1account = U256::from(1);
        let nonce = 0u64;
        let account_index = 0u32;
        let l1_tx_hash = U256::from(1);

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = whitelisted_caller();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
        Swap::<T>::set_key(caller_origin.clone(), pubkey)?;
        let nonce = nonce + 1;

        // Add balance
        BalanceMap::insert((account_index, token_index), amount);

        let mut command = [0u8; 81];
        command[0] = OP_WITHDRAW;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&token_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount.to_be_bytes());
        command[49..81].copy_from_slice(&l1account.to_be_bytes());

        let signature = BabyJubjub::sign(&command, &key);
        let mut sign = [0u8; 64];
        sign.copy_from_slice(&[signature.r.encode(), signature.s.encode()].concat());

    }: withdraw(RawOrigin::Signed(caller), sign, token_index, amount, l1account, nonce)

    swap_benchmark {
        let pool_index = 0u32;
        let reverse = 0u8;
        let amount = U256::from(1);
        let nonce = 0u64;
        let account_index = 0u32;

        let token_index_0 = 10u32;
        let token_index_1 = 20u32;
        let pool_amount_0 = U256::from(100);
        let pool_amount_1 = U256::from(100);

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = whitelisted_caller();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
        Swap::<T>::set_key(caller_origin.clone(), pubkey)?;
        let nonce = nonce + 1;

        // Add pool
        PoolIndexCount::set(pool_index + 1);
        PoolIndexMap::insert((token_index_0, token_index_1), pool_index);
        PoolMap::insert(pool_index, (token_index_0, token_index_1, pool_amount_0, pool_amount_1));

        // Add balance
        BalanceMap::insert((account_index, token_index_0), amount);

        let mut command = [0u8; 81];
        command[0] = OP_SWAP;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&U256::from(reverse).to_be_bytes());
        command[49..81].copy_from_slice(&amount.to_be_bytes());

        let signature = BabyJubjub::sign(&command, &key);
        let mut sign = [0u8; 64];
        sign.copy_from_slice(&[signature.r.encode(), signature.s.encode()].concat());
    }: swap(RawOrigin::Signed(caller), sign, pool_index, reverse, amount, nonce)

    pool_supply_benchmark {
        let pool_index = 0u32;
        let amount0 = U256::from(1);
        let amount1 = U256::from(1);
        let nonce = 0u64;
        let account_index = 0u32;

        let token_index_0 = 10u32;
        let token_index_1 = 20u32;
        let pool_amount_0 = U256::from(100);
        let pool_amount_1 = U256::from(100);

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = whitelisted_caller();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
        Swap::<T>::set_key(caller_origin.clone(), pubkey)?;
        let nonce = nonce + 1;

        // Add pool
        PoolIndexCount::set(pool_index + 1);
        PoolIndexMap::insert((token_index_0, token_index_1), pool_index);
        PoolMap::insert(pool_index, (token_index_0, token_index_1, pool_amount_0, pool_amount_1));

        // Add balance
        BalanceMap::insert((account_index, token_index_0), amount0);
        BalanceMap::insert((account_index, token_index_1), amount1);

        let mut command = [0u8; 81];
        command[0] = OP_SUPPLY;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount0.to_be_bytes());
        command[49..81].copy_from_slice(&amount1.to_be_bytes());

        let signature = BabyJubjub::sign(&command, &key);
        let mut sign = [0u8; 64];
        sign.copy_from_slice(&[signature.r.encode(), signature.s.encode()].concat());
    }: pool_supply(RawOrigin::Signed(caller), sign, pool_index, amount0, amount1, nonce)

    pool_retrieve_benchmark {
        let pool_index = 0u32;
        let amount0 = U256::from(1);
        let amount1 = U256::from(1);
        let nonce = 0u64;
        let account_index = 0u32;

        let token_index_0 = 10u32;
        let token_index_1 = 20u32;
        let pool_amount_0 = U256::from(100);
        let pool_amount_1 = U256::from(100);

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = whitelisted_caller();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
        Swap::<T>::set_key(caller_origin.clone(), pubkey)?;
        let nonce = nonce + 1;

        // Add pool
        PoolIndexCount::set(pool_index + 1);
        PoolIndexMap::insert((token_index_0, token_index_1), pool_index);
        PoolMap::insert(pool_index, (token_index_0, token_index_1, pool_amount_0, pool_amount_1));

        // Add share
        ShareMap::insert((account_index, pool_index), amount0 + amount1);

        let mut command = [0u8; 81];
        command[0] = OP_RETRIEVE;
        command[1..9].copy_from_slice(&nonce.to_be_bytes());
        command[9..13].copy_from_slice(&account_index.to_be_bytes());
        command[13..17].copy_from_slice(&pool_index.to_be_bytes());
        command[17..49].copy_from_slice(&amount0.to_be_bytes());
        command[49..81].copy_from_slice(&amount1.to_be_bytes());

        let signature = BabyJubjub::sign(&command, &key);
        let mut sign = [0u8; 64];
        sign.copy_from_slice(&[signature.r.encode(), signature.s.encode()].concat());
    }: pool_retrieve(RawOrigin::Signed(caller), sign, pool_index, amount0, amount1, nonce)

    ack {
        let req_id_start = U256::from(0);
        let batch_size = 10u64;

        for i in 0u64..batch_size {
            let req_id = req_id_start + 1 + i;
            let op = Ops::Deposit(U256::from(0), U256::from(0), U256::from(0), i.into(), 0u32, 0u32, U256::from(0), U256::from(0), 0u32);
            PendingReqMap::insert(&req_id, op);
            DepositMap::insert(&req_id, U256::from(0));
        }

        let caller: T::AccountId = whitelisted_caller();
    }: ack(RawOrigin::Signed(caller), req_id_start)
}
