import { createApp } from "vue";
import App from "./App.vue";
import axios from "axios";

import "bulma/css/bulma.css";
import router from "./router";
import store from "./store";

axios.interceptors.response.use(
  function (response) {
    return response;
  },
  function (error) {
    return Promise.reject(error);
  }
);

createApp(App).use(store).use(router).mount("#app");
