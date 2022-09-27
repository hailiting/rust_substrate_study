#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	use sp_runtime::{
		offchain::storage::{
			MutateStorageError,	StorageRetrievalError,StorageValueRef
		},
		traits::Zero
	};
	use serde::{Deserialize,};
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxPrices: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}


	#[pallet::error]
	pub enum Error<T> {
	}
	// storage
	#[pallet::storage]
	#[pallet::getter(fn prices)]
	pub(super) type Prices<T: Config> = StorageValue<_, BoundedVec<u32, T::MaxPrices>,ValueQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_local_storage(
			origin: OriginFor<T>,
			some_number: u32,
		)->DispatchResultWithPostInfo {
		 	ensure_signed(origin)?;
			Self::add_price(some_number);
			Ok(().into())
		}

	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>{
		fn offchain_worker(_block_number: T::BlockNumber){
			let value = Self::get_price();
			if let Some(number) = value {
				log::info!("----price---- {:?}", number);
			}
		}
	}

	impl<T: Config> Pallet<T> {
		fn add_price(price: u32){
			<Prices<T>>::mutate(|prices| {
				if prices.try_push(price).is_err(){
					prices[(price % T::MaxPrices::get()) as usize] = price;
				}
			})
		}
		 fn get_price() -> Option<u32>{
			let prices = <Prices<T>>::get();
			if prices.is_empty(){
				None
			} else {
				Some(prices.iter().fold(0_u32, |a,b| a.saturating_add(*b)) as u32)
			}
		}
	}

}
