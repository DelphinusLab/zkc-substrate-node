#![cfg(feature = "runtime-benchmarks")]

use crate::*;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use sp_std::{boxed::*, vec, vec::*};
use crate::Module as Swap;

benchmarks! {
    add_pool_benchmark {

        let nonce = 1u64;
        let token_index_0 = 10u32;
        let token_index_1 = 20u32;

        let key = [2u8; 32];
        let pubkey = BabyJubjub::pubkey_from_secretkey(&key);
        let pubkey = BabyJubjubPoint::encode(&pubkey);

        let caller: T::AccountId = <T as Config>::ADMIN1::get();
        let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));

        NonceMap::<T>::insert(&caller, nonce);
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
}
