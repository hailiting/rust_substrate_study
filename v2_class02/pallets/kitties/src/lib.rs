#![cfg_attr(not(feature="std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use frame_support::traits::Randomness;
  use sp_io::hashing::blake2_128;
  use sp_runtime::traits::{
    Member,
    MaybeSerializeDeserialize,
    AtLeast32BitUnsigned, 
    MaybeDisplay,
    MaybeMallocSizeOf,
    Bounded
  };
  use sp_std::{fmt::Debug};
// type KittyIndex = u32;


  #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
  pub struct Kitty(pub [u8; 16]);

  #[pallet::config]
  pub trait Config: frame_system::Config {
      type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
      type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
      type KittyIndex: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaybeDisplay
			+ AtLeast32BitUnsigned
			+ Default
			+ Bounded
			+ Copy
			+ sp_std::hash::Hash
			+ sp_std::str::FromStr
			+ MaybeMallocSizeOf
			+ MaxEncodedLen
			+ TypeInfo;

  }

  
  #[pallet::type_value]
  pub fn GetDefaultValue<T: Config>() -> T::KittyIndex {
    0u32.into()
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery, GetDefaultValue<T>>;



  #[pallet::storage]
  #[pallet::getter(fn kitties)]
  pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

  #[pallet::storage]
  #[pallet::getter(fn kitty_owner)]
  pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T:Config> {
    KittyCreated(T::AccountId, T::KittyIndex, Kitty),
    KittyBreed(T::AccountId, T::KittyIndex, Kitty),
    KittyTransfered(T::AccountId, T::AccountId, T::KittyIndex)
  }

  #[pallet::error]
  pub enum Error<T>{
    KittiesCountOverflow,
    NotOwner,
    SameKittyId,
    InvalidKittyId,
  }
  #[pallet::call]
  impl<T:Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn create(origin:OriginFor<T>)->DispatchResult {
      let who = ensure_signed(origin)?;
      let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
      let dna = Self::random_value(&who);
      let kitty = Kitty(dna);
      Self::created(who, kitty_id, kitty);
      Ok(())
    }
    #[pallet::weight(10_000)]
    pub fn breed(origin:OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2:T::KittyIndex)->DispatchResult{
      let who = ensure_signed(origin)?;
      ensure!(kitty_id_1!=kitty_id_2, Error::<T>::SameKittyId);
      let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
      let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

      let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
      let selector = Self::random_value(&who);

      let mut data = [0u8; 16];
      for i in 0..kitty_1.0.len(){
        data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i]&!selector[i]);
      }
      let new_kitty = Kitty(data);
      Self::deposit_event(Event::KittyBreed(who.clone(), kitty_id, new_kitty.clone()));
      Self::created(who, kitty_id, new_kitty);
      Ok(())
    }
    #[pallet::weight(10_000)]
    pub fn transfer(origin:OriginFor<T>,kitty_id: T::KittyIndex, new_owner:T::AccountId)->DispatchResult {
      let who = ensure_signed(origin)?;
      Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
      ensure!(Self::kitty_owner(kitty_id)==Some(who.clone()), Error::<T>::NotOwner);
      <KittyOwner<T>>::insert(kitty_id, &new_owner);
      Self::deposit_event(Event::KittyTransfered(who, new_owner, kitty_id));
      Ok(())
    }
  }

  impl<T: Config> Pallet<T>{
    fn random_value(sender: &T::AccountId)->[u8; 16] {
      let payload = (
        T::Randomness::random_seed(),
        &sender,
        <frame_system::Pallet::<T>>::extrinsic_index()
      );
      payload.using_encoded(blake2_128)
    }
    fn get_next_id() -> sp_std::result::Result<T::KittyIndex, DispatchError> {
      let _max_value_local = T::KittyIndex::max_value();
      let next_kitty_id = Self::next_kitty_id();
      if next_kitty_id == _max_value_local {
        return Err(Error::<T>::KittiesCountOverflow.into());
      }
      Ok(next_kitty_id)
    }

    fn get_kitty(kitty_id: T::KittyIndex)->Result<Kitty,()>{
      match Self::kitties(kitty_id){
        Some(kitty)=>Ok(kitty),
        None=>Err(())
      }
    }
    fn created(who: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty){
      Kitties::<T>::insert(kitty_id, kitty.clone());
      KittyOwner::<T>::insert(kitty_id, who.clone());
      NextKittyId::<T>::set(kitty_id+1u32.into());
      Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
    }
  }
}