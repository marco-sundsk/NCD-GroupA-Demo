<template>
  <div>
    <div class="vld-parent">
      <loading :active.sync="isLoading" :can-cancel="true" :is-full-page="fullPage"></loading>
    </div>
    <main>
      <h1 class="shadow py-2" v-show="isSignedIn">
        Welcome {{ accountId }}, You have {{ this.leftCount }} times to dice
      </h1>

      <div class="contianer">
        <div class="row">
          <div class="col-md-3">
            <form v-on:submit.prevent="buyDice" class="shadow mt-5 py-4">
              <fieldset ref="fieldset">
                <div class="form-group py-3">
                  <span class="text-white">Buy </span>
                  <select name="rollCount" v-model="rollCount" id="roll" class="ml-2 mr-2">
                    <option value="1" selected="selected">1</option>
                    <option value="5">5</option>
                    <option value="10">10</option>
                    <option value="30">30</option>
                    <option value="50">50</option>
                    <option value="100">100</option>
                    <option value="1000">1000</option>
                  </select> 
                  <span class="text-white"> times</span>
                  <button id="buy_dice" class="btn btn-danger btn-sm ml-2">
                    Buy
                  </button>
                </div>
              </fieldset>
            </form>
          </div>
          <div class="col-md-6">
            <div class="mt-5">
              <h2 class="text-white text-center">Jackpot:</h2>
              <p class="text-center"><span class="display-3" style="letter-spacing: 1rem">{{jackpot}}</span>Near</p>
              <p class="text-center display-4 text-white">Recent Wins</p>
              <table class="table table-hover" style="border: solid 1px #dee2e6;background: #fff">
                <thead>
                  <tr>
                    <th scope="col">Height</th>
                    <th scope="col">Username</th>
                    <th scope="col">Prize</th>
                    <!-- <th scope="col">TS</th> -->
                  </tr>
                </thead>
                <tbody>
                  <tr v-for="item in winList" :key="item.height">
                    <th scope="row">{{item.height}}</th>
                    <td>{{item.user}}</td>
                    <td><span style="color: green">{{formatAmount(item.amount)}}</span> Near</td>
                    <!-- <td>{{item.ts}}</td> -->
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
          <div class="col-md-3">
            <form v-on:submit.prevent="rollDice" class="shadow mt-5 py-4">
              <fieldset ref="fieldset">
                <div>
                  <span class="text-white">Select number to bet</span><br>
                  <ul>
                    <li class="number-item"><a :class="active1" @click="chooseNumber(1)">1</a></li>
                    <li class="number-item"><a :class="active2" @click="chooseNumber(2)">2</a></li>
                    <li class="number-item"><a :class="active3" @click="chooseNumber(3)">3</a></li>
                    <li class="number-item"><a :class="active4" @click="chooseNumber(4)">4</a></li>
                    <li class="number-item"><a :class="active5" @click="chooseNumber(5)">5</a></li>
                    <li class="number-item"><a :class="active6" @click="chooseNumber(6)">6</a></li>
                  </ul>
                  <button id="roll_dice"  class="btn btn-warning btn-sm ml-2">
                    Roll
                  </button>            
                </div>
              </fieldset>
            </form>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script>
import { logout } from "../utils";
// Import component
import Loading from 'vue-loading-overlay';
// Import stylesheet
import 'vue-loading-overlay/dist/vue-loading.css';

export default {
  name: "SignedIn",

  components: {
    Loading
  },
  data: function () {
    return {
      gas: Math.pow(10, 14).toString(),
      at: "000000000000000000000000",
      savedGreeting: "",
      newGreeting: "",
      leftCount: 0,
      rollCount: "",
      rollNumber: 1,
      jackpot: 100,
      winList: {},
      active1: "active",
      active2: "",
      active3: "",
      active4: "",
      active5: "",
      active6: "",
      isLoading: false,
      fullPage: true
    };
  },

  created() {
    this.getLeftCount()
    this.getWinHistory()
    this.getContactInfo()
  },

  computed: {
    isSignedIn() {
      return window.walletConnection
        ? window.walletConnection.isSignedIn()
        : false;
    },
    accountId() {
      return window.accountId;
    },
    contractId() {
      return window.contract ? window.contract.contractId : null;
    },
    networkId() {
      return window.networkId;
    },
  },

  methods: {
    chooseNumber(num) {
      console.log(num)
      this.rollNumber = num
      if (num==1) {
        this.active1 = 'active'
        this.active2 = ''
        this.active3 = ''
        this.active4 = ''
        this.active5 = ''
        this.active6 = ''
      }
      if (num==2) {
        this.active1 = ''
        this.active2 = 'active'
        this.active3 = ''
        this.active4 = ''
        this.active5 = ''
        this.active6 = ''
      }
      if (num==3) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = 'active'
        this.active4 = ''
        this.active5 = ''
        this.active6 = ''
      }
      if (num==4) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = ''
        this.active4 = 'active'
        this.active5 = ''
        this.active6 = ''
      }
      if (num==5) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = ''
        this.active4 = ''
        this.active5 = 'active'
        this.active6 = ''
      }
      if (num==6) {
        this.active1 = ''
        this.active2 = ''
        this.active3 = ''
        this.active4 = ''
        this.active5 = ''
        this.active6 = 'active'
      }
    },
    getLeftCount() {
      //retrieve greeting
      window.contract
        .get_account_dice_count({ account_id: window.accountId })
        .then((leftCount) => {
          this.leftCount = leftCount;
        });
    },
    showDice: function (num) {
      this.diceNum=num
      this.diceStart = !this.diceStart;
    },

    buyDice: async function () {
      // fired on form submit button used to update the greeting

      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true;

      try {
        // make an update call to the smart contract
        await window.contract.buy_dice(
          {},
          this.gas,
          parseInt(this.rollCount) + this.at
        );
      } catch (e) {
        console.log(e);
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false;
      }
    },

    rollDice: async function () {
      this.isLoading = true;
      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true;

      try {
        // make an update call to the smart contract
        await window.contract
          .roll_dice({
            target: parseInt(this.rollNumber),
          })
          .then((res) => {
            this.isLoading = false;
            this.getWinHistory();
            if (res.dice_point === res.user_guess) {
              const reward_amount = res.reward_amount.toString();
              const temp_amount = reward_amount.substr(
                0,
                reward_amount.length - 20
              );
              const int_part = temp_amount.substr(0, temp_amount.length - 4);
              const float_part = temp_amount.substr(0, int_part.legnth);
              alert(
                "Congratulations to you, you win " +
                    int_part +
                    "." +
                    float_part +
                    " near"
              );
            } else {
              alert("You lose, the number is " + res.dice_point);
            }
            this.leftCount = this.leftCount - 1;
            this.jackpot = this.formatAmount(res.jackpod_left);
            console.log(res);
          });
      } catch (e) {
        console.log(e); //re-throw
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false;
      }
    },

    getWinHistory: async function () {
      try {
        // make an update call to the smart contract
        await window.contract
          .get_win_history({
            from_index: 0,
            limit: 20,
          })
          .then((res) => {
            this.winList = res;
            console.log(res);
          });
      } catch (e) {
        console.log(e); //re-throw
      }
    },

    getContactInfo: async function () {
      try {
        // make an update call to the smart contract
        await window.contract
          .get_contract_info({})
          .then((res) => {
            this.jackpot = this.formatAmount(res.jack_pod);
            console.log(res);
          });
      } catch (e) {
        console.log(e); //re-throw
      }
    },

    formatAmount: function (amount) {
      const reward_amount = amount.toString();
      const temp_amount = reward_amount.substr(
        0,
        reward_amount.length - 20
      );
      const int_part = temp_amount.substr(0, temp_amount.length - 4);
      const float_part = temp_amount.substr(0, int_part.legnth);
      return int_part + "." +float_part;
    },
    logout: logout,
  },
};
</script>
