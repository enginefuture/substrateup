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
