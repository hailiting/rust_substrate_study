#![cfg_attr(not(feature="std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature="runtime-benchmarks")]
mod Benchmarking;
pub mod weights;

extern crate frame_support;
extern crate frame_system;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*
	};

	use frame_system::pallet_prelude::*;
	pub use crate::weights::WeightInfo;
	use sp_std::vec::Vec;


	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type MaxClaimLength: Get<u32>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn proofs)]
	pub type Proofs<T:Config> = StorageMap<
		_,
		Blake2_128Concat, // key加密方式
		Vec<u8>,
		(T::AccountId, T::BlockNumber)
	>;


	#[pallet::event]
	#[pallet::metadata(T::AccountId="AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransfered(T::AccountId, T::AccountId,Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
		ClaimTooLarge,
	}

	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T>{
		// #[pallet::weight(0)]
		#[pallet::weight(T::WeightInfo::create_claim_benchmark(claim.to_vec()))]
		pub fn create_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>
		)-> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
			ensure!(
				T::MaxClaimLength::get() >= claim.len() as u32,
				Error::<T>::ClaimTooLarge
			);
			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number())
			);
			Self::deposit_event(Event::ClaimCreated(sender, claim));
			Ok(().into())
		}


		#[pallet::weight(0)]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&claim);
			Self::deposit_event(Event::ClaimRevoked(sender, claim));
			Ok(().into())
		}
		#[pallet::weight(0)]
		pub fn transfer_claim(
			origin:OriginFor<T>,
			receiver: T::AccountId,
			claim: Vec<u8>
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			// Proofs::<T>::remove(&claim);
			let current_block = <frame_system::Pallet<T>>::block_number();
			// 直接覆盖
			Proofs::<T>::insert(&claim, (receiver.clone(), current_block));

			// 改变值的方法，除了insert 还可以 mutate
			// Proofs::<T>::mutate(&claim, |value|{
			// 	let mut v = value.as_mut().unwrap();
			// 	v.0 = receiver.clone();
			// 	v.1 = frame_system::Pallet::<T>::block_number();
			// });
			Self::deposit_event(Event::ClaimTransfered(sender, receiver, claim));
			Ok(().into())
		}
	}
}