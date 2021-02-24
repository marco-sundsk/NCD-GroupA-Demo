NCD-GroupA-Demo
==================

This [Vue] app was initialized with [create-near-app]

RollingDice
====================

User deposits 1 NEAR and given a number between 1 and 6. Then he get a chance to roll a dice on chain.
if he rolls out a number identical with the number he just given, then he wins the jackpot, otherwise, his 1 NEAR will be rushed into the jackpot.

Contract Interface
====================

```rust
/// contract commission rate when winner get prized
pub struct RewardFeeFraction {
    pub numerator: u32,
    pub denominator: u32,
}

/// each winner's highlight record
pub struct HumanReadableWinnerInfo {
    pub user: AccountId,
    pub amount: U128,
    pub height: U64,
    pub ts: U64,
}

/// status of this contract except commission fee info
pub struct HumanReadableContractInfo {
    pub owner: AccountId,
    pub jack_pod: U128,  // winner gain half of jack_pod
    pub owner_pod: U128, // the contract commission fee goes to here
    pub dice_number: u8, // how many dice we use in one rolling action
    pub rolling_fee: U128,  // how much does one rolling action cost
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
/// otherwise, the jackpod is initialized to 0.
#[payable]
pub fn deposit_jackpod(&mut self);

/// withdraw ownerpod to owner's account
pub fn withdraw_ownerpod(&mut self, amount: U128);

/// Updates current reward fee fraction to the new given fraction.
pub fn update_reward_fee_fraction(&mut self, reward_fee_fraction: RewardFeeFraction);

/// Updates current dice number in one rolling action.
pub fn update_dice_number(&mut self, dice_number: u8);

/// Updates current rolling fee needed for one rolling action.
pub fn update_rolling_fee(&mut self, rolling_fee: U128);

//**************************/
//***** USER FUNCTIONS *****/
//**************************/

/// user deposit near to play rolling once, left over amount would be paid back.
/// and if the target equals to the dice point, he get half of the jackpod,
/// otherwise, the rolling_fee amount of NEAR in her deposit would be rush into jackpod.
/// return the total dice points.
#[payable]
pub fn roll_dice(&mut self, target: u8) -> u8;


//**************************/
//***** USER FUNCTIONS *****/
//**************************/

/// get a list of winner's record.
pub fn get_win_history(&self, from_index: u64, limit: u64) -> Vec<HumanReadableWinnerInfo>;

/// get current contract status
pub fn get_contract_info(&self) -> HumanReadableContractInfo;

/// get current contract commission fee
pub fn get_reward_fee_fraction(&self) -> RewardFeeFraction;

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
