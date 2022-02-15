#![allow(clippy::ref_in_deref)]

pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{self as sim, call};

pub mod utils;

#[test]
fn test_counter() {
    let root = sim::init_simulator(None);
    let counter = utils::setup_counter(&root);

    let res = call!(root, counter.increment());
    let val: u32 = res.unwrap_json();
    assert_eq!(val, 1);
}
