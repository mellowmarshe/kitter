import { createStore } from "vuex";

import userModule from "./modules/user";
import authModule from "./modules/auth";
import postsModule from "./modules/posts";
import errorsModule from "./modules/errors";

export default createStore({
  state: {},
  mutations: {},
  actions: {},
  modules: {
    user: userModule,
    posts: postsModule,
    auth: authModule,
    errors: errorsModule,
  },
});
