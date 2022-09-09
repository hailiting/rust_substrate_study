use crate::{mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_works(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_who = KittyOwner::<Test>::get(0);
		assert_eq!(kitty_who, Some(1));
		let next_kitty_count = NextKittyId::<Test>::get();
		assert_eq!(next_kitty_count, 1);
		let kitty = Kitties::<Test>::get(0).unwrap();
		let expected_event = super::Event::KittyCreated(1, 0, kitty);
		assert_eq!(
			System::events()[1].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn create_no_enough_balance(){
	new_test_ext().execute_with(||{
			assert_noop!(
				KittiesModule::create(Origin::signed(7)), 
				Error::<Test>::NoEnoughBalance
			);
	});
}

#[test]
fn bread_works(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		let kitty_id_2:<Test as Config>::KittyIndex = 1u32.into();

		let kitty_id_3:<Test as Config>::KittyIndex = 2u32.into();

		assert_ok!(KittiesModule::bread(Origin::signed(1), kitty_id_1.clone(), kitty_id_2.clone()));
		
		let kitty = Kitties::<Test>::get(kitty_id_3).unwrap();
		let expected_event = super::Event::KittyBread(1, kitty_id_3, kitty.clone());
		let expected_create_event = super::Event::KittyCreated(1, kitty_id_3, kitty);

		System::assert_has_event(mock::Event::KittiesModule(expected_event));
		System::assert_has_event(mock::Event::KittiesModule(expected_create_event));

		let kitty_who = KittyOwner::<Test>::get(kitty_id_3);
		assert_eq!(kitty_who, Some(1));

		let next_kitty_count = NextKittyId::<Test>::get();
		assert_eq!(next_kitty_count, 3u32);
	});
}
#[test]
fn bread_invalid_kitty_id(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		let kitty_id_2:<Test as Config>::KittyIndex = 3u32.into();

		assert_noop!(
			KittiesModule::bread(Origin::signed(1), kitty_id_1, kitty_id_2),
			Error::<Test>::InvalidKittyId
		);
	});
}
#[test]
fn bread_no_enough_balance(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		assert_ok!(KittiesModule::create(Origin::signed(2)));
		
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		let kitty_id_2:<Test as Config>::KittyIndex = 1u32.into();

		assert_noop!(
			KittiesModule::bread(Origin::signed(2), kitty_id_1, kitty_id_2),
			Error::<Test>::NoEnoughBalance
		);
	});
}
#[test]
fn bread_not_owner(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		let kitty_id_2:<Test as Config>::KittyIndex = 1u32.into();

		assert_noop!(
			KittiesModule::bread(Origin::signed(2), kitty_id_1, kitty_id_2), 
			Error::<Test>::NotOwner
		);
	});
}
#[test]
fn bread_same_kitty_id(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();

		assert_noop!(
			KittiesModule::bread(Origin::signed(1), kitty_id_1, kitty_id_1), 
			Error::<Test>::SameKittyId
		);
	});
}

#[test]
fn transfer_works(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();

		assert_ok!(
			KittiesModule::transfer(
				Origin::signed(1), 
				kitty_id_1.clone(), 
				2
			)
		);

		let kitty_who = KittyOwner::<Test>::get(kitty_id_1);
		assert_eq!(kitty_who, Some(2));

		let next_kitty_count = NextKittyId::<Test>::get();
		assert_eq!(next_kitty_count, 1u32);


		let expected_event = super::Event::KittyTransfered(1, 2, 0);
		System::assert_has_event(
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn transfer_invalid_kitty_id(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_id_1:<Test as Config>::KittyIndex = 1u32.into();

		assert_noop!(
			KittiesModule::transfer(
				Origin::signed(1), 
				kitty_id_1, 
				2
			),
			Error::<Test>::InvalidKittyId
		);
	});
}


#[test]
fn transfer_not_owner(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();

		assert_noop!(
			KittiesModule::transfer(
				Origin::signed(2), 
				kitty_id_1, 
				3
			),
			Error::<Test>::NotOwner
		);
	});
}
#[test]
fn sell_works(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();

		assert_ok!(
			KittiesModule::sell(
				Origin::signed(1), 
				kitty_id_1.clone(), 
				Some(200000000000000000)
			)
		);


		let price = OnSale::<Test>::get(kitty_id_1).unwrap();
		assert_eq!(price.clone(), 200000000000000000);

		let expected_event = super::Event::OnSaleEvent(1, 0, Some(price));
		assert_eq!(
			System::events()[2].event,
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn sell_not_owner(){
	new_test_ext().execute_with(||{
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();

		assert_noop!(
			KittiesModule::sell(
				Origin::signed(2), 
				kitty_id_1, 
				Some(200000000000000000)
			),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn buy_works(){
	new_test_ext().execute_with(||{
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(
			KittiesModule::sell(
				Origin::signed(1), 
				kitty_id_1.clone(), 
				Some(200000000000)
			)
		);
		assert_ok!(
			KittiesModule::buy(
				Origin::signed(2), 
				kitty_id_1.clone(), 
			)
		);

		let kitty_who = KittyOwner::<Test>::get(0);
		assert_eq!(kitty_who, Some(2));

		let next_kitty_count = NextKittyId::<Test>::get();
		assert_eq!(next_kitty_count, 1);

		let expected_event = super::Event::SoldEvent(1, 2, kitty_id_1, 200000000000u128);
		System::assert_has_event(
			mock::Event::KittiesModule(expected_event)
		);
	});
}

#[test]
fn buy_no_enough_balance(){
	new_test_ext().execute_with(||{
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_ok!(
			KittiesModule::sell(
				Origin::signed(1), 
				kitty_id_1.clone(), 
				Some(20000000000000000000000000)
			)
		);
		assert_noop!(
			KittiesModule::buy(
				Origin::signed(2), 
				kitty_id_1.clone(), 
			),
			Error::<Test>::NoEnoughBalance
		);
	});
}
#[test]
fn buy_not_for_sale(){
	new_test_ext().execute_with(||{
		let kitty_id_1:<Test as Config>::KittyIndex = 0u32.into();
		assert_ok!(KittiesModule::create(Origin::signed(1)));
		assert_noop!(
			KittiesModule::buy(
				Origin::signed(2), 
				kitty_id_1.clone(), 
			),
			Error::<Test>::NotForSale
		);
	});
}