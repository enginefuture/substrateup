## 1.编译运行通过
![image](https://github.com/enginefuture/substrateup/blob/main/class2/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE1.png)

## 2.KittyIndex不在pallet中指定，而是在runtime里面绑定
```rust
///runtime绑定 Kittyindex
/// runtime lib.rs
impl pallet_kitties::Config for Runtime {
	type Event = Event;
	type Randomness = RandomnessCollectiveFlip;
	type KittyIndex = u32;
}
///palllet kitties lib.rs
 #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type KittyIndex: Copy + Default + Bounded + AtLeast32BitUnsigned + Parameter + MaxEncodedLen;
        
    }

```

## 3.扩展存储，能得到一个账号拥有的所有kitties
```rust
#[pallet::storage]
#[pallet::getter(fn owner_all_kitties)]
pub type OwnerAllKitties<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<T::KittyIndex,ConstU32<256>>, ValueQuery>;
```
## 4.create和breed需要质押一定数量的token，在transfer的时候能转移质押。
```rust
 type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

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
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type KittyStake: Get<BalanceOf<Self>>;
        
    }

    ...
     #[pallet::weight(10_100)]
        pub fn breed(origin: OriginFor<T>, kitty_id_1:T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResult {
            let who = ensure_signed(origin)?;

            //check kitty_id
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
            T::Currency::reserve(&who, T::KittyReserve::get()).map_err(|_| Error::<T>::TokenNotEnough)?;
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

            OwnerAllKitties::<T>::try_mutate(&who, |kitties_i_vec| {
				kitties_i_vec.try_push(new_kitty.clone())
			}).map_err(|_| Error::<T>::OverMaxOwnerKitties)?;


            Self::deposit_event(Event::KittyCreated(who, kitty_id, new_kitty));
            Ok(())
        }


```
## 5.pod js 运行
![image](https://github.com/enginefuture/substrateup/blob/main/class2/2.png)
![image](https://github.com/enginefuture/substrateup/blob/main/class2/3.png)
![image](https://github.com/enginefuture/substrateup/blob/main/class2/4.png)
