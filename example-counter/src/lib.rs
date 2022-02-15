use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Counter {
    val: u32,
}

#[near_bindgen]
impl Counter {
    pub fn increment(&mut self) -> u32 {
        self.val += 1;
        self.val
    }
}
