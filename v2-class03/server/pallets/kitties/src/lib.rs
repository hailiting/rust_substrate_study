#![cfg_attr(not(feature="std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;


#[frame_support::pallet]
pub mod pallet {
  // use frame_support::pallet_prelude::*;
  // use frame_support::traits::Randomness;
  use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits:: {
      Currency,
      Randomness,
      ReservableCurrency,
      BalanceStatus,
    }
  };
  use frame_system::pallet_prelude::*;
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

  type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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
		#[pallet::constant]
    type MoneyForCreateKitty: Get<BalanceOf<Self>>;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
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

  #[pallet::storage]
  #[pallet::getter(fn on_sale)]
  pub type OnSale<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn all_kitties)]
  pub type AllKitties<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::KittyIndex, ConstU32<{u32::MAX}>>>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T:Config> {
    KittyCreated(T::AccountId, T::KittyIndex, Kitty),
    KittyBread(T::AccountId, T::KittyIndex, Kitty),
    KittyTransfered(T::AccountId, T::AccountId, T::KittyIndex),
    OnSaleEvent(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
    SoldEvent(T::AccountId, T::AccountId, T::KittyIndex, BalanceOf<T>),
  }

  #[pallet::error]
  pub enum Error<T>{
    KittiesCountOverflow,
    NotOwner,
    SameKittyId,
    InvalidKittyId,
    NoEnoughBalance,
    NotForSale,
  }
  #[pallet::call]
  impl<T:Config> Pallet<T> {
    #[pallet::weight(0)]
    pub fn create(origin:OriginFor<T>)->DispatchResult {
      let who = ensure_signed(origin)?;
      let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
      let dna = Self::random_value(&who);
      let kitty = Kitty(dna);
      T::Currency::reserve(&who, T::MoneyForCreateKitty::get()).map_err(|_| Error::<T>::NoEnoughBalance)?;
      Self::created(who, kitty_id, kitty);
      Ok(())
    }
    #[pallet::weight(0)]
    pub fn bread(origin:OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2:T::KittyIndex)->DispatchResult{
      let who = ensure_signed(origin)?;
      ensure!(kitty_id_1!=kitty_id_2, Error::<T>::SameKittyId);
      let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
      let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

      Self::is_owner(who.clone(), kitty_id_1).map_err(|_| Error::<T>::NotOwner)?;
      Self::is_owner(who.clone(), kitty_id_2).map_err(|_| Error::<T>::NotOwner)?;
      let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;
      let selector = Self::random_value(&who);

      let mut data = [0u8; 16];
      for i in 0..kitty_1.0.len(){
        data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i]&!selector[i]);
      }
      let new_kitty = Kitty(data);
      T::Currency::reserve(&who, T::MoneyForCreateKitty::get()).map_err(|_| Error::<T>::NoEnoughBalance)?;
      Self::deposit_event(Event::KittyBread(who.clone(), kitty_id, new_kitty.clone()));
      Self::created(who, kitty_id, new_kitty);
      Ok(())
    }
    #[pallet::weight(0)]
    pub fn transfer(origin:OriginFor<T>, kitty_id: T::KittyIndex, new_owner:T::AccountId)->DispatchResult {
      let who = ensure_signed(origin)?;
      Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
      // ensure!(Self::kitty_owner(kitty_id)==Some(who.clone()), Error::<T>::NotOwner);
      Self::is_owner(who.clone(), kitty_id).map_err(|_| Error::<T>::NotOwner)?;
      T::Currency::repatriate_reserved(&who, &new_owner,  T::MoneyForCreateKitty::get(), BalanceStatus::Reserved).map_err(|_| Error::<T>::NoEnoughBalance)?;
      Self::transferred(who, new_owner, kitty_id);
      Ok(())
    }
    #[pallet::weight(0)]
    // 卖  account  price kitty_id
    pub fn sell(
      origin:OriginFor<T>,
      kitty_id: T::KittyIndex,
      price: Option<BalanceOf<T>>
    ) -> DispatchResult {
      let who = ensure_signed(origin)?;
      Self::is_owner(who.clone(), kitty_id).map_err(|_| Error::<T>::NotOwner)?;
      OnSale::<T>::insert(kitty_id, price);
      Self::deposit_event(Event::OnSaleEvent(who, kitty_id, price));
      Ok(())
    }
    // 买
    #[pallet::weight(0)]
    pub fn buy(
      origin:OriginFor<T>,
      kitty_id: T::KittyIndex,
    ) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let owner = Self::get_owner(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
      let price = Self::get_price(kitty_id).map_err(|_| Error::<T>::NotForSale)?;
      // 取 存
      // T::Currency::reserve(&who, price).map_err(|_| Error::<T>::NoEnoughBalance)?;
      // T::Currency::unreserve(&owner, price);
      T::Currency::transfer(
        &who,
        &owner,
        price,
        frame_support::traits::ExistenceRequirement::KeepAlive,
      ).map_err(|_| Error::<T>::NoEnoughBalance)?;
      OnSale::<T>::remove(kitty_id);
      Self::transferred(owner.clone(), who.clone(), kitty_id);
      Self::deposit_event(Event::SoldEvent(owner, who, kitty_id, price));
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
    fn get_next_id() -> sp_std::result::Result<T::KittyIndex, ()> {
      let _max_value_local = T::KittyIndex::max_value();
      let next_kitty_id = Self::next_kitty_id();
      if next_kitty_id == _max_value_local {
        return Err(());
      }
      Ok(next_kitty_id)
    }

    fn is_owner(who: T::AccountId, kitty_id: T::KittyIndex)->Result<(),()>{
      let _kitty_owner = Self::kitty_owner(kitty_id);
      if _kitty_owner == Some(who.clone()) {
       return Ok(());
      }
      Err(())
    }
    fn get_owner(kitty_id: T::KittyIndex)->Result<T::AccountId,()>{
      match Self::kitty_owner(kitty_id){
        Some(owner)=>Ok(owner),
        None=>Err(())
      }
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
      match Self::all_kitties(who.clone()) {
        Some(mut kitty_id_list) => {
          kitty_id_list.try_push(kitty_id).unwrap();
          AllKitties::<T>::insert(who.clone(), kitty_id_list);
        },
        None =>{
          let mut kitty_id_list = BoundedVec::<T::KittyIndex, ConstU32<{u32::MAX}>>::with_max_capacity();
          kitty_id_list.try_push(kitty_id).unwrap();
          AllKitties::<T>::insert(who.clone(), kitty_id_list);
        }
      }
      Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
    }
    fn transferred(old_owner:T::AccountId, new_owner: T::AccountId, kitty_id: T::KittyIndex){
      KittyOwner::<T>::insert(kitty_id, new_owner.clone());
      Self::deposit_event(Event::KittyTransfered(old_owner.clone(), new_owner.clone(), kitty_id.clone()));

      let mut kitty_id_list = AllKitties::<T>::get(old_owner.clone()).unwrap();
      let mut index= 0;
      for (i,id) in kitty_id_list.iter().enumerate(){
        if id == &kitty_id{
          index = i;
          break;
        }
      }
      kitty_id_list.remove(index);
      AllKitties::<T>::insert(old_owner.clone(), kitty_id_list);

      let mut kitty_id_list = if let Some(kitty_id_list) = Self::all_kitties(new_owner.clone()) {
        kitty_id_list
      } else {
        BoundedVec::<T::KittyIndex, ConstU32<{u32::MAX}>>::with_max_capacity()
      };
      kitty_id_list.try_push(kitty_id).unwrap();
      AllKitties::<T>::insert(new_owner.clone(), kitty_id_list);
    }
    fn get_price(kitty_id: T::KittyIndex)->Result<BalanceOf<T>,()>{
      match Self::on_sale(kitty_id){
        Some(price)=>Ok(price),
        None=>Err(())
      }
    }
  }
}