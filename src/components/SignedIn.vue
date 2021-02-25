<template>
  <div>
    <button class="link" style="float: right" v-on:click="logout">Sign out</button>
    <main>
      <h1>
        Welcome {{ accountId }}, You have {{this.leftCount}} time to dice
      </h1>
      <form v-on:submit.prevent="buyDice">
        <fieldset ref="fieldset">
          <label
            for="buyDice"
            style="display:block; color:var(--gray);margin-bottom:0.5em;"
          >Buy Dice</label>
          <div style="display:flex">
            Buy<input v-model="rollCount" autocomplete="off" id="roll" style="flex:1" />times
            <button id="buy_dice" style="border-radius:0 5px 5px 0">Buy</button>
          </div>
        </fieldset>
      </form>

      <form v-on:submit.prevent="rollDice">
        <fieldset ref="fieldset">
          <label
            for="rollDice"
            style="display:block; color:var(--gray);margin-bottom:0.5em;"
          >Roll Dice</label>
          <div style="display:flex">
            <input v-model="rollNumber" autocomplete="off" id="roll" style="flex:1" />
            <button id="roll_dice" style="border-radius:0 5px 5px 0">Roll</button>
          </div>
        </fieldset>
      </form>
    </main>

    <Notification
      v-show="notificationVisible"
      ref="notification"
      :networkId="networkId"
      :msg="'called method: set_greeting'"
      :contractId="contractId"
      :visible="false"
    />
  </div>
</template>

<script>
import { logout } from "../utils"

import Notification from "./Notification.vue"

export default {
  name: "SignedIn",

  components: {
    Notification,
  },

  data: function () {
    return {
      gas:Math.pow(10,14).toString(),
      at:"000000000000000000000000",
      savedGreeting: "",
      newGreeting: "",
      leftCount:0,
      rollCount:"",
      rollNumber:"",
      notificationVisible: false,
    }
  },

  created() {
    this.getLeftCount();
  },

  computed: {
    isSignedIn() {
      return window.walletConnection? window.walletConnection.isSignedIn(): false
    },
    accountId() {
      return window.accountId
    },
    contractId() {
      return window.contract? window.contract.contractId: null
    },
    networkId() {
      return window.networkId
    },
  },

  methods: {
    getLeftCount() {
      //retrieve greeting
      window.contract
        .get_account_dice_count({ account_id: window.accountId })
        .then((leftCount) => {
          this.leftCount = leftCount
        })
    },

    buyDice: async function () {
      // fired on form submit button used to update the greeting

      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true

      try {
        
        // make an update call to the smart contract
        await window.contract.buy_dice(
          {},
          this.gas,
          parseInt(this.rollCount)+this.at
        )
      } catch (e) {
        console.log(e)
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false
      }

      this.notificationVisible = true //show new notification

      // remove Notification again after css animation completes
      // this allows it to be shown again next time the form is submitted
      setTimeout(() => {
        this.notificationVisible = false
      }, 11000)

    },

    rollDice: async function () {
      // fired on form submit button used to update the greeting

      // disable the form while the value gets updated on-chain
      this.$refs.fieldset.disabled = true

      try {
        
        // make an update call to the smart contract
        await window.contract.roll_dice(
          {
            target: parseInt(this.rollNumber),
          }
        ).then((res)=>{
          if (res.dice_point===res.user_guess) {
            const reward_amount = res.reward_amount.toString()
            const temp_amount = reward_amount.substr(0,reward_amount.length-20)
            const int_part = temp_amount.substr(0,temp_amount.length-4)
            const float_part = temp_amount.substr(0,int_part.legnth)
            alert('Congratulations to you, you win '+int_part+'.'+float_part+' near')
          } else {
            alert('You lose, the number is '+res.dice_point)
          }
          this.leftCount = this.leftCount-1
          console.log(res)
        });
      } catch (e) {
        console.log(e) //re-throw
      } finally {
        // re-enable the form, whether the call succeeded or failed
        this.$refs.fieldset.disabled = false
      }

      this.notificationVisible = true //show new notification

      // remove Notification again after css animation completes
      // this allows it to be shown again next time the form is submitted
      setTimeout(() => {
        this.notificationVisible = false
      }, 11000)

    },

    logout: logout,
  },
}
</script>
