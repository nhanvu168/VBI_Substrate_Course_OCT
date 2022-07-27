use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(DemoModule::create_student(Origin::signed(1), b"nhanvh_fabbi".to_vec(), 28));
		// Read pallet storage and assert an expected result.
		assert_eq!(DemoModule::student_id(), 1);
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(DemoModule::create_student(Origin::signed(1),  b"nhanvh_fabbi".to_vec(), 18), Error::<Test>::TooYoung);
	});
}
