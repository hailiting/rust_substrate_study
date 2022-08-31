use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
// 创建存证
	// 成功
	// 已存在而失败
// 撤销存证
	// 成功
	// 不存在而失败
// 转移存证
	// 成功
	// 失败
		// owner!=sender
		// 不存在存证

#[test]
fn create_claim_works(){
  // new_test_ext 初始化测试环境
  new_test_ext().execute_with(|| {
    let claim = vec![0,1];
    assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
    let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
    assert_eq!(
      Proofs::<Test>::get(&bounded_claim),
      Some((1, frame_system::Pallet::<Test>::block_number()))
    );
  });
}

#[test]
fn create_claim_failed_when_claim_already_exist(){
  new_test_ext().execute_with(|| {
    let claim:Vec<u8> = vec![0,1];
    let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
    assert_noop!(
      PoeModule::create_claim(Origin::signed(1),claim.clone()),
      Error::<Test>::ProofAlreadyExist
    );
  });
}

#[test]
fn revoke_claim_works(){
  new_test_ext().execute_with(|| {
    let claim = vec![0,1];
    let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
    assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
  });
}
#[test]
fn revoke_claim_failed_when_claim_is_not_exist(){
  new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	});
}
#[test]
fn revoke_claim_failed_when_claim_sender_is_not_owener(){
	new_test_ext().execute_with(|| {
    let claim = vec![0,1];
    let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
    assert_noop!(
      PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
    );
  });
}

#[test]
fn transfer_claim_works(){
  new_test_ext().execute_with(|| {
    let claim = vec![0,1];
    let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
    assert_ok!(
      PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2)
    );
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
    assert_eq!(
      Proofs::<Test>::get(&bounded_claim),
      Some((2, frame_system::Pallet::<Test>::block_number()))
    );
  });
}
#[test]
fn transfer_claim_failed_when_claim_is_not_exist(){
  new_test_ext().execute_with(|| {
		let claim = vec![0,1];
		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	});
}
#[test]
fn transfer_claim_failed_when_claim_sender_is_not_owener(){
	new_test_ext().execute_with(|| {
    let claim = vec![0,1];
    let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
    assert_noop!(
      PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
    );
  });
}