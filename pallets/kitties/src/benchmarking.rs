//! Benchmarking setup for pallet-kitties

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;


benchmarks! {
	// Name of benchmark : create_kitty
	create_kitty {
		let price = 100u32.into();
		let caller: T::AccountId = whitelisted_caller();
	}: create_kitty(RawOrigin::Signed(caller), price)
	verify {
		assert_eq!(KittyQuantity::<T>::get(), 1);
	}
	// Name of benchmark : change_owner
	change_owner {
		let nhanfabbi: T::AccountId = whitelisted_caller();
		let nhanvhfabbi: T::AccountId = account("nhanvhfabbi",0,0);
		let price = 100u32.into();
		Kitties::<T>::create_kitty(RawOrigin::Signed(nhanfabbi.clone()).into(), price)?;
		let nhanfabbi_kitty_dna = KittyOwner::<T>::get(&nhanfabbi).ok_or(Error::<T>::NoneValue).unwrap()[0];
	}: change_owner(RawOrigin::Signed(nhanfabbi.clone()), nhanfabbi_kitty_dna, nhanvhfabbi.clone())

	verify {
		assert!(KittyOwner::<T>::get(&nhanvhfabbi).is_some());
	}


	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
