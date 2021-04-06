import axios from "axios";
import config from "../../config";

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
    const res = await axios.get(`${config.url}/api/user/me`, {
      headers: {
        "Content-type": "application/json",
        Authorization: `Bearer ${rootState.auth.token}`,
      },
    });

    commit("setUser", res.data);
  },
  async registerUser({ rootState }, login) {
    await axios.post(`${config.url}/api/user/register`, JSON.stringify(login), {
      headers: {
        "Content-type": "application/json",
      },
    });

    console.log(rootState);
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
