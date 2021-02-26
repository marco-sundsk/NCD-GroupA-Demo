NCD-GroupA-Demo
==================

This is a homework demo project for NCD program phase-1.

Rolling Dice On NEAR
====================

Guys, let's roll dice on NEAR.  

## Why dice

Randomness is always a key focus on any blockchain. We wanna show you how convenient that a random number can get on NEAR blockchain.  
  
To achieve that, it is hard to believe there is a better way than to make a dice dapp.  

Beyond what you can see in this demo, NEAR can even generate independent randomness not per block, but per receipt!

## How to play

On home page, user can see the whole status of playground without login, i.e. an NEAR account is not necessary. He would have full imformation about owner account of this contract, dice price, commission fee rate, the size of current jackpod and etc.  

Then, user can login with NEAR account and buy several dices. With dices bought, he can guess a number and roll dice again and again. If the dice point is equal to his guess, half of jackpod would belong to him. Otherwise the amount he paid for the dice would belong to the jackpod.  

During playing, the latest 20 win records would appear and be auto refreshed on screen too. 

About Contract
====================
It's need to be mentioned that it is a pure dapp project, which means there is no centralized backend nor data server, all persistent information is stored and mananged on NEAR chain by a contract.

## Contract Structure

```rust
/// This structure describe a winning event
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WinnerInfo {
    pub user: AccountId,  // winner accountId
    pub amount: Balance,  // how much he got as win reward
    pub height: BlockHeight,  // the block hight this event happened
    pub ts: u64,  // the timestamp this event happened
}

/// main structure of this contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NearDice {
    pub owner_id: AccountId,  // owner can adjust params of this playground
    pub dice_number: u8,  // how many dices one rolling event uses
    pub rolling_fee: Balance,  // how much a dice costs when user buys it
    pub jack_pod: Balance,  // as name shows and half of it would belong to the winner
    pub owner_pod: Balance,  // winner would share a tip to the playground, this is where those tips go
    pub reward_fee_fraction: RewardFeeFraction,  // a fraction defines tip rate
    // an always grow vector records all win event, 
    // as a demo, we ignore the management of its size, 
    // but in real project, it must be taken care of, 
    // maybe has a maximum length and remove the oldest item when exceeds.
    pub win_history: Vector<WinnerInfo>,
    // records dice user bought by his payment amount. 
    // This map has a mechanism to shrink, 
    // when a user's balance is reduce to zero, the entry would be removed.
    pub accounts: LookupMap<AccountId, Balance>,
}

```

## Contract Interface


```rust
/// winner's tip rate
pub struct RewardFeeFraction {
    pub numerator: u32,
    pub denominator: u32,
}

/// a human readable version for win event struct, used in return value to caller
pub struct HumanReadableWinnerInfo {
    pub user: AccountId,  // winner accountId
    pub amount: U128,  // the reward he got
    pub height: U64,  // block height the event happens
    pub ts: U64,  // timestamp the event happens
}

/// status of this playground, as return value of get_contract_info
pub struct HumanReadableContractInfo {
    pub owner: AccountId,  // who runs this playground, if you feel bad, just sue him :)
    pub jack_pod: U128,  // you know what it means
    pub owner_pod: U128, // winner's tip goes to here, owner can withdraw
    pub dice_number: u8, // how many dice we use in one rolling event
    pub rolling_fee: U128,  // how much a dice costs when user wanna buy it
}

/// every roll_dice event would return this info
pub struct HumanReadableDiceResult {
    pub user: AccountId,  // who rolls
    pub user_guess: u8,  // the number he guess
    pub dice_point: u8,  // the number dice shows
    pub reward_amount: U128,  // reward he got
    pub jackpod_left: U128,  // jackpod after this event
    pub height: U64,  // the block height when he rolls
    pub ts: U64,  // the timestamp when he rolls
}

//****************/
//***** INIT *****/
//****************/

/// initialization of this contract
 #[init]
pub fn new(
   owner_id: AccountId,
   dice_number: u8,
   rolling_fee: U128,
   reward_fee_fraction: RewardFeeFraction,
) -> Self;


//***************************/
//***** OWNER FUNCTIONS *****/
//***************************/

/// deposit to jackpod, used for initalizing the very first jackpod,
/// otherwise, the jackpod is initialized as 0.
#[payable]
pub fn deposit_jackpod(&mut self);

/// withdraw ownerpod to owner's account
pub fn withdraw_ownerpod(&mut self, amount: U128);

/// Updates current reward fee fraction to the new given fraction.
pub fn update_reward_fee_fraction(&mut self, reward_fee_fraction: RewardFeeFraction);

/// Updates current dice number used in one rolling event.
pub fn update_dice_number(&mut self, dice_number: u8);

/// Updates current dice price.
pub fn update_rolling_fee(&mut self, rolling_fee: U128);

//**************************/
//***** USER FUNCTIONS *****/
//**************************/

/// user deposit near to buy dice. 
/// he can buy multiple dices,
/// any leftover amount would refund
/// eg: rolling_fee is 1 Near, he can buy_dice with 4.4 Near and got 4 dices and 0.4 Near refund.
#[payable]
pub fn buy_dice(&mut self);

/// user roll dice once, then his available dice count would reduce by one.
pub fn roll_dice(&mut self, target: u8) -> HumanReadableDiceResult;


//**************************/
//***** VIEW FUNCTIONS *****/
//**************************/

/// get a list of winn events in LIFO order
/// best practise is set from_index to 0, and limit to 20,
/// that means to get latest 20 win events information with latest first order.
pub fn get_win_history(&self, from_index: u64, limit: u64) -> Vec<HumanReadableWinnerInfo>;

/// get current playground status
pub fn get_contract_info(&self) -> HumanReadableContractInfo;

/// get current winner tip rate
pub fn get_reward_fee_fraction(&self) -> RewardFeeFraction;

/// get account's available dice count
pub fn get_account_dice_count(&self, account_id: String) -> u8;

```

Quick Start
===========

To run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `yarn install`
3. Run the local development server: `yarn dev` (see `package.json` for a
   full list of `scripts` you can run with `yarn`)

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


Exploring The Code
==================

1. The "backend" code lives in the `/contract` folder. See the README there for
   more info.
2. The frontend code lives in the `/src` folder. `/src/main.js` is a great
   place to start exploring.
3. Tests: there are different kinds of tests for the frontend and the smart
   contract. See `contract/README` for info about how it's tested. The frontend
   code gets tested with [jest]. You can run both of these at once with `yarn
   run test`.


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `yarn dev`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-------------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `NCD-GroupA-Demo.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `NCD-GroupA-Demo.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account NCD-GroupA-Demo.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: set contract name in code
---------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'NCD-GroupA-Demo.YOUR-NAME.testnet'


Step 3: deploy!
---------------

One command:

    yarn deploy

As you can see in `package.json`, this does two things:

1. builds & deploys smart contract to NEAR TestNet
2. builds & deploys frontend code to GitHub using [gh-pages]. This will only work if the project already has a repository set up on GitHub. Feel free to modify the `deploy` script in `package.json` to deploy elsewhere.


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [Vue]: https://vuejs.org/
  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
