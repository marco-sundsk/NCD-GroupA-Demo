/*
 * This is NearDice contract:
 * 
 * 
 *
 */

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::wee_alloc;
use near_sdk::json_types::{U64, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, BlockHeight, Promise};
use std::collections::HashMap;
use near_sdk::collections::Vector;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardFeeFraction {
    pub numerator: u32,
    pub denominator: u32,
}

impl RewardFeeFraction {
    pub fn assert_valid(&self) {
        assert_ne!(self.denominator, 0, "Denominator must be a positive number");
        assert!(
            self.numerator <= self.denominator,
            "The reward fee must be less or equal to 1"
        );
    }

    pub fn multiply(&self, value: Balance) -> Balance {
        (U256::from(self.numerator) * U256::from(value) / U256::from(self.denominator)).as_u128()
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WinnerInfo {
    pub user: AccountId,  // winner
    pub amount: Balance, // win prize
    pub height: BlockHeight,
    pub ts: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableWinnerInfo {
    pub user: AccountId,
    pub amount: U128,
    pub height: U64,
    pub ts: U64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableContractInfo {
    pub owner: AccountId,
    pub jack_pod: U128,
    pub owner_pod: U128,
    pub dice_number: u8,
    pub rolling_fee: U128,
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearDice {
    pub owner_id: AccountId,
    pub dice_number: u8,
    pub rolling_fee: Balance,  // how many NEAR needed to roll once.
    pub jack_pod: Balance,  // half of them would be show to user as jack_pod amount
    pub owner_pod: Balance,  // incoming of the contract, can be withdraw by owner
    pub reward_fee_fraction: RewardFeeFraction,
    pub win_history: Vector<WinnerInfo>,
    records: HashMap<String, String>,  // obsolete when release
}

impl Default for NearDice {
    fn default() -> Self {
        env::panic(b"dice contract should be initialized before usage")
    }
}

#[near_bindgen]
impl NearDice {

    #[init]
    pub fn new(
        owner_id: AccountId,
        dice_number: u8,
        rolling_fee: U128,
        reward_fee_fraction: RewardFeeFraction,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        reward_fee_fraction.assert_valid();
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );
        
        Self {
            owner_id,
            dice_number,
            rolling_fee: rolling_fee.into(),
            jack_pod: 0_u128,
            owner_pod: 0_u128,
            reward_fee_fraction,
            win_history: Vector::new(b"w".to_vec()),
            records: HashMap::new(),
        }
    }

    //***********************/
    // owner functions
    //***********************/

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Can only be called by the owner"
        );
    }
    /// 
    pub fn withdraw_ownerpod(&mut self, amount: U128) {
        self.assert_owner();
        let amount: Balance = amount.into();
        assert!(
            self.owner_pod >= amount,
            "The owner pod has insurficent funds"
        );

        let account_id = env::predecessor_account_id();
        self.owner_pod -= amount;
        Promise::new(account_id).transfer(amount);
    }

    #[payable]
    pub fn deposit_jackpod(&mut self) {
        self.assert_owner();
        let amount = env::attached_deposit();
        self.jack_pod += amount;
    }

    /// Owner's method.
    /// Updates current reward fee fraction to the new given fraction.
    pub fn update_reward_fee_fraction(&mut self, reward_fee_fraction: RewardFeeFraction) {
        self.assert_owner();
        reward_fee_fraction.assert_valid();
        self.reward_fee_fraction = reward_fee_fraction;
    }

    pub fn update_dice_number(&mut self, dice_number: u8) {
        self.assert_owner();
        self.dice_number = dice_number;
    }

    pub fn update_rolling_fee(&mut self, rolling_fee: U128) {
        self.assert_owner();
        self.rolling_fee = rolling_fee.into();
    }

    //***********************/
    // rolling functions
    //***********************/

    /// rolling dice
    /// check the deposit is larger than rolling_fee NEAR, and return leftover back to caller at end of call,
    /// add rolling_fee NEAR to jackpod and get random number between [1, self.dice_number * 6],
    /// if identical to target, modify jackpod amount and transfer half of jackpod to caller (within a tip to the owner_pod)
    #[payable]
    pub fn roll_dice(&mut self, target: u8) -> u8 {

        // check called by real user NOT from other contracts
        let account_id = env::predecessor_account_id();
        assert_eq!(
            account_id.clone(),
            env::signer_account_id(),
            "This method must be called directly from user."
        );

        // check user attached enough rolling fee
        let amount = env::attached_deposit();
        assert!(
            amount >= self.rolling_fee,
            format!("You must deposit more than {}", self.rolling_fee)
        );

        // leftover will return to caller
        let leftover = amount - self.rolling_fee;
        // net_reward is user's actual reward
        let net_reward;
        // always update jack_pod before rolling dice
        self.jack_pod += self.rolling_fee;

        // rolling dice here
        let random_u8: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
        let dice_point = self.dice_number as u16 * 6_u16 * random_u8 as u16 / 0x100_u16 + 1;
        
        // let's see how lucky caller is this time
        if target == dice_point as u8 {  // Wow, he wins
            // figure out gross reward and update jack pod
            let gross_reward = self.jack_pod / 2;
            self.jack_pod -= gross_reward;
            // split gross to net and owner fee
            let owners_fee = self.reward_fee_fraction.multiply(gross_reward);
            net_reward = gross_reward - owners_fee;
            // update owner pod 
            self.owner_pod += owners_fee;
            // records this winning
            self.win_history.push(&WinnerInfo {
                user: account_id.clone(),
                amount: net_reward,
                height: env::block_index(),
                ts: env::block_timestamp(),
            });
        } else {  // oops, he fails
            net_reward = 0;
        }

        // refund caller if needed
        let refound = net_reward + leftover;
        if refound > 0 {
            Promise::new(account_id.clone()).transfer(net_reward+leftover);
        }
        
        // return dice point
        dice_point as u8
    }

    pub fn set_greeting(&mut self, message: String) {
        let account_id = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("Saving greeting '{}' for account '{}'", message, account_id,).as_bytes());

        self.records.insert(account_id, message);
    }

    //***********************/
    // view functions
    //***********************/

    fn get_hr_info(&self, index: u64) -> HumanReadableWinnerInfo {
        let info = self.win_history.get(index).expect("Error: no this item in winner history!");
        HumanReadableWinnerInfo {
            user: info.user.clone(),
            amount: info.amount.into(),
            height: info.height.into(),
            ts: info.ts.into(),
        }
    }

    /// Returns the list of winner info in LIFO order
    pub fn get_win_history(&self, from_index: u64, limit: u64) -> Vec<HumanReadableWinnerInfo> {
        let counts: u64 = self.win_history.len() as u64;
        (from_index..std::cmp::min(from_index + limit, counts))
            .map(|index| self.get_hr_info(counts - index - 1))  // reverse to get LIFO order
            .collect()
    }

    pub fn get_contract_info(&self) -> HumanReadableContractInfo {
        HumanReadableContractInfo {
            owner: self.owner_id.clone(),
            jack_pod: self.jack_pod.into(),
            owner_pod: self.owner_pod.into(),
            dice_number: self.dice_number,
            rolling_fee: self.rolling_fee.into(),
        }
    }

    /// Returns the current reward fee as a fraction.
    pub fn get_reward_fee_fraction(&self) -> RewardFeeFraction {
        self.reward_fee_fraction.clone()
    }

    // `match` is similar to `switch` in other languages; here we use it to default to "Hello" if
    // self.records.get(&account_id) is not yet defined.
    // Learn more: https://doc.rust-lang.org/book/ch06-02-match.html#matching-with-optiont
    pub fn get_greeting(&self, account_id: String) -> &str {
        match self.records.get(&account_id) {
            Some(greeting) => greeting,
            None => "Hello",
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = NearDice::default();
        contract.set_greeting("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_greeting("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_greeting() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = NearDice::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(
            "Hello".to_string(),
            contract.get_greeting("francis.near".to_string())
        );
    }
}
