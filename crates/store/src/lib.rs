use dashmap::DashMap;
use dex_core::PoolPrice;

#[derive(Clone)]
pub struct HotStore {
    map: DashMap<(u32, u32, u16), PoolPrice>,
}

impl HotStore {
    pub fn new() -> Self {
        Self { map: DashMap::new() }
    }

    pub fn update(&self, price: PoolPrice) {
        self.map.insert((price.token_in, price.token_out, price.dex_id), price);
    }

    pub fn get_pair_prices(&self, pair: (u32, u32)) -> Vec<PoolPrice> {
        self.map
            .iter()
            .filter(|entry| (entry.key().0, entry.key().1) == pair)
            .map(|e| *e.value())
            .collect()
    }
}
