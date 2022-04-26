#![allow(unused_parens)]
#![allow(unused_imports)]
use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;
use sp_std::vec::Vec;

pub trait WeightInfo {
  fn create_claim_benchmark(s:Vec<u8>,)->Weight;
}
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
  fn create_claim_benchmark(_s: Vec<u8>,)->Weight {
    (54_000_000 as Weight)
      .saturating_add(T::DbWeight::get().reads(1 as Weight))
      .saturating_add(T::DbWeight::get().writes(1 as Weight))
  }
}
impl WeightInfo for(){
  fn create_claim_benchmark(_s:Vec<u8>,)->Weight{
    (54_000_000 as Weight)
      .saturating_add(RocksDbWeight::get().reads(1 as Weight))
      .saturating_add(RocksDbWeight::get().writes(1 as Weight))
  }
}