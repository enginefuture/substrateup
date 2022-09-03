```rust
use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok,BoundedVec};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));

		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();

		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}


#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		let _ = PoeModule::revoke_claim(Origin::signed(1), claim.clone());
		assert_ne!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0; 1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn revoke_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0; 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(||{
		let claim = vec![0,1];
		let bounded_claim = BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		let _ = PoeModule::transfer_claim(Origin::signed(1), 2, claim.clone());
		assert_ne!(
			Proofs::<Test>::get(&bounded_claim),
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	});
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0; 1];
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 1, claim.clone()),
            Error::<Test>::ClaimNotExist
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_not_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0; 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(3), 2, claim.clone()),
            Error::<Test>::NotClaimOwner
        );
    })
}

#[test]
fn transfer_claim_failed_when_sender_is_destination() {
    new_test_ext().execute_with(||{
        let claim = vec![0; 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), 1, claim.clone()),
            Error::<Test>::DestinationIsClaimOwner
        );
    })
}
```
![image](https://github.com/enginefuture/substrateup/blob/main/class1/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE.png)
