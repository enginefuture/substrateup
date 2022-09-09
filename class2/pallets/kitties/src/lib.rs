#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use sp_io::hashing::blake2_128;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Bounded};




    // #[pallet::type_value]
    // pub fn GetDefaultValue() -> T::KittyIndex {
    //     0_u32
    // }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type KittyIndex: Copy + Default + Bounded + AtLeast32BitUnsigned + Parameter + MaxEncodedLen;
        
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    ///定义kitty存储，valuequery，或者optionquery
    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type NextKittyId<T:Config> = StorageValue<_,T::KittyIndex>;


    ///blake2做hash映射
    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T:Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;
    

    ///kitty 所有者的账号id
    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;


    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated(T::AccountId, T::KittyIndex, Kitty),
        KittyBred(T::AccountId, T::KittyIndex, Kitty),
        KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,
        NotOwner,
        SameKittyId,
        KittiesOverflow,
    }
    ///创建kitty
    #[pallet::call]
    impl <T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

            let dna = Self::random_value(&who);
            let kitty = Kitty(dna);
            
            Kitties::<T>::insert(kitty_id, &kitty);
            KittyOwner::<T>::insert(kitty_id, &who);
            NextKittyId::<T>::put(kitty_id + 1u32.into());

            //emit an event
            Self::deposit_event(Event::KittyCreated(who, kitty_id, kitty));
            Ok(())
        }

        #[pallet::weight(10_100)]
        pub fn breed(origin: OriginFor<T>, kitty_id_1:T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResult {
            let who = ensure_signed(origin)?;

            //check kitty_id
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
            let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
            let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

            // get next id
            let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

            //selector for breeding
            let selector = Self::random_value(&who);

            let mut data = [0u8; 16];
            for i in 0..kitty_1.0.len() {
                //0 choose kitty2, and 1 chosse kitty1
                data[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
            }

            let new_kitty = Kitty(data);
            <Kitties<T>>::insert(kitty_id, &new_kitty);
            KittyOwner::<T>::insert(kitty_id, &who);
            NextKittyId::<T>::put(kitty_id + 1u32.into());

            Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn transfer(origin: OriginFor<T>, kitty_id: T::KittyIndex, new_owner: T::AccountId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;
            ensure!(Self::kitty_owner(kitty_id) == Some(who.clone()), Error::<T>::NotOwner);
            <KittyOwner<T>>::insert(kitty_id, &new_owner);
            Self::deposit_event(Event::KittyTransferred(who, new_owner, kitty_id));
            Ok(())
        }
    }



    ///跟数据相关获取kitty数据
    impl <T: Config> Pallet<T> {
        //get a random 256.生成dna
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                //相当于以太坊的nounce
                <frame_system::Pallet::<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }

        //get next id
        fn get_next_id() -> Result<T::KittyIndex, ()> {
            //next_kitty_id由上面宏自动生成
            match Self::next_kitty_id() {
                Some(val) => {
					// ensure!(val != T::KittyIndex::max_value(), Error::<T>::KittiesOverflow);
					Ok(val)
				}
				None => Ok(1u32.into()),
            }
        }

        //get kitty via id
        fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()> {
            match Self::kitties(kitty_id) {
                Some(kitty) => Ok(kitty),
                None => Err(()),
            }
        }
        
    }
}