//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Ternoa-Recommended-Reference-Machine`, CPU: `AMD EPYC 7281 16-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("alphanet-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/ternoa
// benchmark
// pallet
// --chain=alphanet-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output
// ./output

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
        // Storage: Identity Registrars (r:1 w:1)
        /// The range of component `r` is `[1, 19]`.
        fn add_registrar(r: u32, ) -> Weight {
                Weight::from_ref_time(66_019_000 as RefTimeWeight)
                        // Standard Error: 135_000
                        .saturating_add(Weight::from_ref_time(1_459_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity IdentityOf (r:1 w:1)
        /// The range of component `r` is `[1, 20]`.
        /// The range of component `x` is `[1, 100]`.
        fn set_identity(r: u32, x: u32, ) -> Weight {
                Weight::from_ref_time(106_441_000 as RefTimeWeight)
                        // Standard Error: 253_000
                        .saturating_add(Weight::from_ref_time(1_917_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        // Standard Error: 50_000
                        .saturating_add(Weight::from_ref_time(1_097_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity IdentityOf (r:1 w:0)
        // Storage: Identity SubsOf (r:1 w:1)
        // Storage: Identity SuperOf (r:1 w:1)
        /// The range of component `s` is `[1, 100]`.
        fn set_subs_new(s: u32, ) -> Weight {
                Weight::from_ref_time(74_029_000 as RefTimeWeight)
                        // Standard Error: 117_000
                        .saturating_add(Weight::from_ref_time(8_002_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
        }
        // Storage: Identity IdentityOf (r:1 w:0)
        // Storage: Identity SubsOf (r:1 w:1)
        // Storage: Identity SuperOf (r:0 w:1)
        /// The range of component `p` is `[1, 100]`.
        fn set_subs_old(p: u32, ) -> Weight {
                Weight::from_ref_time(67_006_000 as RefTimeWeight)
                        // Standard Error: 77_000
                        .saturating_add(Weight::from_ref_time(4_934_000 as RefTimeWeight).scalar_saturating_mul(p as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(p as RefTimeWeight)))
        }
        // Storage: Identity SubsOf (r:1 w:1)
        // Storage: Identity IdentityOf (r:1 w:1)
        // Storage: Identity SuperOf (r:0 w:100)
        /// The range of component `r` is `[1, 20]`.
        /// The range of component `s` is `[1, 100]`.
        /// The range of component `x` is `[1, 100]`.
        fn clear_identity(_r: u32, s: u32, x: u32, ) -> Weight {
                Weight::from_ref_time(288_283_000 as RefTimeWeight)
                        // Standard Error: 78_000
                        .saturating_add(Weight::from_ref_time(3_825_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        // Standard Error: 78_000
                        .saturating_add(Weight::from_ref_time(392_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
        }
        // Storage: Identity Registrars (r:1 w:0)
        // Storage: Identity IdentityOf (r:1 w:1)
        /// The range of component `r` is `[1, 20]`.
        /// The range of component `x` is `[1, 100]`.
        fn request_judgement(r: u32, x: u32, ) -> Weight {
                Weight::from_ref_time(159_653_000 as RefTimeWeight)
                        // Standard Error: 243_000
                        .saturating_add(Weight::from_ref_time(227_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        // Standard Error: 48_000
                        .saturating_add(Weight::from_ref_time(913_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity IdentityOf (r:1 w:1)
        /// The range of component `r` is `[1, 20]`.
        /// The range of component `x` is `[1, 100]`.
        fn cancel_request(r: u32, x: u32, ) -> Weight {
                Weight::from_ref_time(95_298_000 as RefTimeWeight)
                        // Standard Error: 222_000
                        .saturating_add(Weight::from_ref_time(1_916_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        // Standard Error: 43_000
                        .saturating_add(Weight::from_ref_time(1_124_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity Registrars (r:1 w:1)
        /// The range of component `r` is `[1, 19]`.
        fn set_fee(r: u32, ) -> Weight {
                Weight::from_ref_time(24_000_000 as RefTimeWeight)
                        // Standard Error: 87_000
                        .saturating_add(Weight::from_ref_time(1_987_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity Registrars (r:1 w:1)
        /// The range of component `r` is `[1, 19]`.
        fn set_account_id(r: u32, ) -> Weight {
                Weight::from_ref_time(29_114_000 as RefTimeWeight)
                        // Standard Error: 117_000
                        .saturating_add(Weight::from_ref_time(2_108_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity Registrars (r:1 w:1)
        /// The range of component `r` is `[1, 19]`.
        fn set_fields(r: u32, ) -> Weight {
                Weight::from_ref_time(26_134_000 as RefTimeWeight)
                        // Standard Error: 99_000
                        .saturating_add(Weight::from_ref_time(1_960_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity Registrars (r:1 w:0)
        // Storage: Identity IdentityOf (r:1 w:1)
        /// The range of component `r` is `[1, 19]`.
        /// The range of component `x` is `[1, 100]`.
        fn provide_judgement(r: u32, x: u32, ) -> Weight {
                Weight::from_ref_time(94_790_000 as RefTimeWeight)
                        // Standard Error: 233_000
                        .saturating_add(Weight::from_ref_time(1_798_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        // Standard Error: 43_000
                        .saturating_add(Weight::from_ref_time(918_000 as RefTimeWeight).scalar_saturating_mul(x as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity SubsOf (r:1 w:1)
        // Storage: Identity IdentityOf (r:1 w:1)
        // Storage: System Account (r:1 w:1)
        // Storage: Identity SuperOf (r:0 w:100)
        /// The range of component `r` is `[1, 20]`.
        /// The range of component `s` is `[1, 100]`.
        /// The range of component `x` is `[1, 100]`.
        fn kill_identity(r: u32, s: u32, _x: u32, ) -> Weight {
                Weight::from_ref_time(315_548_000 as RefTimeWeight)
                        // Standard Error: 384_000
                        .saturating_add(Weight::from_ref_time(153_000 as RefTimeWeight).scalar_saturating_mul(r as RefTimeWeight))
                        // Standard Error: 75_000
                        .saturating_add(Weight::from_ref_time(2_409_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes((1 as RefTimeWeight).saturating_mul(s as RefTimeWeight)))
        }
        // Storage: Identity IdentityOf (r:1 w:0)
        // Storage: Identity SuperOf (r:1 w:1)
        // Storage: Identity SubsOf (r:1 w:1)
        /// The range of component `s` is `[1, 99]`.
        fn add_sub(s: u32, ) -> Weight {
                Weight::from_ref_time(116_683_000 as RefTimeWeight)
                        // Standard Error: 47_000
                        .saturating_add(Weight::from_ref_time(1_235_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: Identity IdentityOf (r:1 w:0)
        // Storage: Identity SuperOf (r:1 w:1)
        /// The range of component `s` is `[1, 100]`.
        fn rename_sub(s: u32, ) -> Weight {
                Weight::from_ref_time(74_514_000 as RefTimeWeight)
                        // Standard Error: 33_000
                        .saturating_add(Weight::from_ref_time(550_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
        }
        // Storage: Identity IdentityOf (r:1 w:0)
        // Storage: Identity SuperOf (r:1 w:1)
        // Storage: Identity SubsOf (r:1 w:1)
        /// The range of component `s` is `[1, 100]`.
        fn remove_sub(s: u32, ) -> Weight {
                Weight::from_ref_time(119_475_000 as RefTimeWeight)
                        // Standard Error: 53_000
                        .saturating_add(Weight::from_ref_time(1_551_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
        // Storage: Identity SuperOf (r:1 w:1)
        // Storage: Identity SubsOf (r:1 w:1)
        /// The range of component `s` is `[1, 99]`.
        fn quit_sub(s: u32, ) -> Weight {
                Weight::from_ref_time(103_803_000 as RefTimeWeight)
                        // Standard Error: 38_000
                        .saturating_add(Weight::from_ref_time(914_000 as RefTimeWeight).scalar_saturating_mul(s as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
                        .saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
        }
}
