const state = {
  errors: [],
};

const getters = {
  getErrors: (state) => state.errors,
};

const actions = {
  async addError({ commit }, error) {
    commit("newError", error);
  },
  async deleteError({ commit }, index) {
    commit("removeError", index);
  },
  async deleteAllErrors({ commit }) {
    commit("removeAllErrors");
  },
};

const mutations = {
  newError: (state, error) => state.errors.unshift(error),
  removeError: (state, index) => state.errors.splice(index, 1),
  removeAllErrors: (state) => (state.errors = []),
};

export default {
  state,
  getters,
  actions,
  mutations,
};
