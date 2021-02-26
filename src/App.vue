<template>
  <div id="root" class="main">
    <div class="bg-dark">
      <b-navbar class="container py-3" type="dark">
        <img :src="require('./assets/neardice-logo.png')" alt="Near Dice" height="32px" class="mr-1">
        <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>
        <b-collapse id="nav-collapse" is-nav>
          <b-navbar-nav>
            <b-nav-item href="#">How To Play</b-nav-item>
            <b-nav-item href="#" disabled>Rules</b-nav-item>
          </b-navbar-nav>

          <!-- Right aligned nav items -->
          <b-navbar-nav class="ml-auto">
            <button class="btn btn-info" style="float: right" v-on:click="logout" v-show="isSignedIn">
              Sign Out
            </button>
            <button class="btn btn-info" style="float: right" v-on:click="login" v-show="!isSignedIn">
              Sign In
            </button>

            <b-nav-item-dropdown text="Language" right>
              <b-dropdown-item href="#">EN</b-dropdown-item>
              <b-dropdown-item href="#">中文</b-dropdown-item>
            </b-nav-item-dropdown>
            <!-- <b-nav-item-dropdown right>
              <template #button-content>
                <em>User</em>
              </template>
              <b-dropdown-item href="#">Profile</b-dropdown-item>
              <b-dropdown-item href="#">Sign Out</b-dropdown-item>
            </b-nav-item-dropdown> -->
          </b-navbar-nav>
        </b-collapse>
      </b-navbar>
    </div>
    <div class="container pt-4">
      <SignedIn />
    </div>

    <footer
      class="bd-footer p-3 p-md-5 mt-5 bg-light text-center text-sm-start"
    >
      <div class="container text-center">
        <p><strong> NEAR DICE DAPP</strong></p>
        <span>Copyright 2021</span>
      </div>
    </footer>
  </div>
</template>

<script>
import "./global.css";
import getConfig from "./config";
import SignedIn from "./components/SignedIn.vue";
import { login,logout } from "./utils";

const nearConfig = getConfig(process.env.NODE_ENV || "development");
console.log(
  `networkId:${nearConfig.networkId} CONTRACT_NAME:${nearConfig.contractName}`
);
window.networkId = nearConfig.networkId;

export default {
  created() {
    document.title = "NEAR DICE DAPP";
  },
  name: "App",
  components: {
    SignedIn,
  },

  computed: {
    isSignedIn() {
      return window.walletConnection.isSignedIn();
    },
  },
  methods: {
    login() {
      console.log("calling utils.login")
      login()
    },
    logout: logout,  
  }
};
</script>

