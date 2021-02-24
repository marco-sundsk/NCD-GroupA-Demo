NCD-GroupA-Demo Smart Contract
==================

A [smart contract] written in [Rust] for an app initialized with [create-near-app]


Quick Start
===========

Before you compile this code, you will need to install Rust with [correct target]

```shell
# deploy
near dev-deploy res/neardice.wasm
# dev-1614170766884-7229574

# init
near call dev-1614170766884-7229574 new '{"owner_id": "humeng.testnet", "dice_number": 1, "rolling_fee": "1000000000000000000000000", "reward_fee_fraction": {"numerator": 5, "denominator": 100}}' --account_id=dev-1614170766884-7229574
```

```shell
near view dev-1614170766884-7229574 get_contract_info ''

near view dev-1614170766884-7229574 get_reward_fee_fraction ''

near view dev-1614170766884-7229574 get_win_history '{"from_index": 0, "limit": 100}'
```

```shell
# deposit very first jackpod with 50 NEAR
near call dev-1614170766884-7229574 deposit_jackpod '' --amount=50 --account_id=humeng.testnet
```

```shell
# roll dice
near call dev-1614170766884-7229574 roll_dice '{"target": 1}' --amount=1 --account_id=humeng.testnet
```


Exploring The Code
==================

1. The main smart contract code lives in `src/lib.rs`. You can compile it with
   the `./compile` script.
2. Tests: You can run smart contract tests with the `./test` script. This runs
   standard Rust tests using [cargo] with a `--nocapture` flag so that you
   can see any debug info you print to the console.


  [smart contract]: https://docs.near.org/docs/roles/developer/contracts/intro
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://github.com/near/near-sdk-rs#pre-requisites
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
