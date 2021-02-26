NCD-GroupA-Demo Smart Contract
==================

A demo contract for NCD Pojrect Phase-1.

Play with this contract
========================
the contract is deployed at testnet with the name `dev-1614240595058-5266655`

you can set it to env for later use:
```shell
export CONTRACTID=dev-1614240595058-5266655
```

## Look around
```shell
# return playground info
near view $CONTRACTID get_contract_info ''
# return winner tip rate
near view $CONTRACTID get_reward_fee_fraction ''
# return win history list
near view $CONTRACTID get_win_history '{"from_index": 0, "limit": 100}'
# return dice count that alice has
near view $CONTRACTID get_account_dice_count '{"account_id": "alice.testnet"}'
```
## Let's play
```shell
# attached 3 Near to buy 3 dices
near call $CONTRACTID buy_dice '' --amount=3 --account_id=alice.testnet
#check user's dice, would return 3 here
near view $CONTRACTID get_account_dice_count '{"account_id": "alice.testnet"}'
# roll dice 3 times, say how luck you are
near call $CONTRACTID roll_dice '{"target": 1}' --account_id=alice.testnet
near call $CONTRACTID roll_dice '{"target": 3}' --account_id=alice.testnet
near call $CONTRACTID roll_dice '{"target": 4}' --account_id=alice.testnet
```

Build Deploy and Init
======================

Before you compile this code, you will need to install Rust with [correct target]


```shell
# building it
srouce ./build.sh
```

```shell
# dev-deploy or deploy it
near dev-deploy res/neardice.wasm

# say it was deploy at $CONTRACTID, then init it 
near call $CONTRACTID new \
  '{"owner_id": "boss.testnet", "dice_number": 1, 
  "rolling_fee": "1000000000000000000000000", 
  "reward_fee_fraction": {"numerator": 5, "denominator": 100}}' \
  --account_id=$CONTRACTID
```

```shell
# last step to open the playgroud is 
# to deposit to the jackpod the very first time
near call $CONTRACTID deposit_jackpod '' --amount=50 --account_id=boss.testnet
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
