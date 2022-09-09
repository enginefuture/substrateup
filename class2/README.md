```rust
///runtime绑定 Kittyindex
impl pallet_kitties::Config for Runtime {
	type Event = Event;
	type Randomness = RandomnessCollectiveFlip;
	type KittyIndex = u32;
}
```
![image](https://github.com/enginefuture/substrateup/blob/main/class2/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE1.png)
