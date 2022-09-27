use crate::{mock::*, Error};
use frame_support::{assert_noop,assert_eq, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		// let some_number = 20u32;
		// let data1 = &some_number.encode().as_slice()
		// let data2 = &some_number.to_le_bytes().to_vec();

		// log::warn!("MaxOwned 1: {:?}", 1);
		// log::info!("count: {:?}",2);
		// log::error!("count own: {:?}", 3);

		assert_ok!(TemplateModule::set_local_storage(Origin::signed(1), 42));
		// // let ddata = data.
		// assert_eq!(20u32,data2.as_ref().unwrap());
		// assert_ok!(TemplateModule::set_local_storage(Origin::signed(1), 42));
	});
}

