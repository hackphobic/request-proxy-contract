use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::ValidAccountId;
use near_sdk::{
    env, near_bindgen, AccountId, Promise
};


#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PaymentProxy {}

impl Default for PaymentProxy {
    fn default() -> Self {
        panic!("Should be initialized before usage")
    }
}

#[near_bindgen]
impl PaymentProxy{

    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        PaymentProxy{}
    }

    #[payable]
    pub fn transfer(&mut self, _to: ValidAccountId, payment_reference: Option<String>) {
        let to: AccountId = _to.into();
        let log_msg = format!(
            "{{beneficiary:{}, amount:{}, payment_reference:{}}}", 
            to, 
            env::attached_deposit(),
            payment_reference.unwrap()
        );

        env::log(log_msg.as_bytes());
        Promise::new(to).transfer(env::attached_deposit());
    }
}