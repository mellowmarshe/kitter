import axios from "axios";

const state = {
  token: "",
};

const getters = {
  getToken: (state) => state.token,
};

const actions = {
  async fetchToken({ commit }, user) {
    const res = await axios.post(
      "http://localhost:8083/api/user/login",
      JSON.stringify(user),
      {
        headers: {
          "Content-type": "application/json",
        },
      }
    );

    commit("setAuth", res.data);
  },
};

const mutations = {
  setAuth: (state, auth) => {
    state.token = auth.token;
  },
};

export default {
  state,
  getters,
  actions,
  mutations,
};
