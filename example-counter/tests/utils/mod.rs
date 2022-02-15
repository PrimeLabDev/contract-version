#![allow(dead_code)]
#![allow(unused_imports)]

use example_counter::CounterContract;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::AccountId;
use near_sdk_sim::transaction::ExecutionStatus;
use near_sdk_sim::ExecutionResult;
use near_sdk_sim::{deploy, ContractAccount, UserAccount};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_WASM_BYTES => "res/example_counter.wasm",
}

pub fn setup_counter(root: &UserAccount) -> ContractAccount<CounterContract> {
    let counter: ContractAccount<CounterContract> = deploy!(
        contract: CounterContract,
        contract_id: "example-counter".to_string(),
        bytes: &COUNTER_WASM_BYTES,
        signer_account: root,
    );

    counter
}

pub fn user(id: u32) -> AccountId {
    format!("user{}", id).parse().unwrap()
}
