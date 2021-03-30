import axios from "axios";

const state = {
  username: "",
  id: NaN,
};

const getters = {
  getUser: (state) => state.username,
  getId: (state) => state.id,
};

const actions = {
  async fetchUser({ commit, rootState }) {
    const res = await axios.get("http://localhost:8083/api/user/me", {
      headers: {
        "Content-type": "application/json",
        Authorization: `Bearer ${rootState.auth.token}`,
      },
    });

    commit("setUser", res.data);
  },
};

const mutations = {
  setUser: (state, user) => {
    state.username = user.username;
    state.id = user.id;
  },
};

export default {
  state,
  getters,
  actions,
  mutations,
};
