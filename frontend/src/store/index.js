import { createStore } from "vuex";

import userModule from "./modules/user";
import postsModule from "./modules/posts";

export default createStore({
  state: {},
  mutations: {},
  actions: {},
  modules: { user: userModule, posts: postsModule },
});
