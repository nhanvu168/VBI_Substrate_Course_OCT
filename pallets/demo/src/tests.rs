use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_for_create_a_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(DemoModule::create_student(Origin::signed(1), b"nhanvh_fabbi".to_vec(), 28));
		// Read pallet storage and assert an expected result.
		assert_eq!(DemoModule::student_id(), 1);
	});
}

#[test]
fn test_error_for_create_student_with_error_too_young() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(DemoModule::create_student(Origin::signed(1),  b"nhanvh_fabbi".to_vec(), 18), Error::<Test>::TooYoung);
	});
}
