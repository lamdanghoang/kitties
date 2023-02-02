//! Benchmarking setup for pallet-kitties

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	create_kitty {

		let caller: T::AccountId = whitelisted_caller();
		let dna = b"HohohohohohohohohohohoPqadpadadad".to_vec();

	}: create_kitty (RawOrigin::Signed(caller), dna, 43)
	verify {
		assert_eq!(TotalKitties::<T>::get(), 1);
	}

	transfer_kitty {
		let dna = b"Hohoh".to_vec();
		let caller: T::AccountId = whitelisted_caller();
		// let caller_origin: <T as frame_system::Config>::RuntimeOrigin = RawOrigin::Signed(caller.clone()).into();

		Kitties::<T>::create_kitty(RawOrigin::Signed(caller.clone()).into(), dna.clone(), 0);
		let kitty = Owner::<T>::get(&caller);

		let receiver: T::AccountId = account("receiver", 0, 0);

	}: transfer_kitty (RawOrigin::Signed(caller), dna, receiver.clone())
	verify {
		assert_eq!(Owner::<T>::get(&receiver), kitty);
	}

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);
}
