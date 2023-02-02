use crate::{mock::*, Error, Event, Gender};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_kitty() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Kitties::create_kitty(
			RuntimeOrigin::signed(1),
			b"Mickey Mouse".to_vec(),
			10u32
		));
		assert_ok!(Kitties::create_kitty(RuntimeOrigin::signed(1), b"Tom Hank".to_vec(), 20u32));
		// Read pallet storage and assert an expected result.
		assert_eq!(Kitties::total_kitties(), 2);
		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::KittyGenerated { dna: b"Tom Hank".to_vec(), owner: 1 }.into(),
		);
	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
