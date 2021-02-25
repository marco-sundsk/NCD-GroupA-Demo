import Vue from "vue"
import { BootstrapVue } from 'bootstrap-vue'
import App from "./App.vue"


// Import Bootstrap an BootstrapVue CSS files (order is important)
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'

// Make BootstrapVue available throughout your project
Vue.use(BootstrapVue)

import { initContract } from "./utils"

Vue.config.productionTip = false

window.nearInitPromise = initContract()
  .then(() => {
    new Vue({
      render: h => h(App),
    }).$mount("#app")
  })
  