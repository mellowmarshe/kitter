import axios from "axios";

const state = {
  posts: [],
};

const getters = {
  getPosts: (state) => state.posts,
};

const actions = {
  async fetchPosts({ commit, rootState }) {
    const res = await axios.post(
      "http://localhost:8083/api/post/posts",
      JSON.stringify({ offset: 0, limit: 25 }),
      {
        headers: {
          "Content-type": "application/json",
          Authorization: `Bearer ${rootState.token}`,
        },
      }
    );
    console.log(res.data);
    commit("setPosts", res.data);
  },
};

const mutations = {
  setPosts: (state, posts) => (state.posts = posts),
  newPost: (state, posts) => state.posts.unshift(posts),
  removePost: (state, id) =>
    (state.posts = state.posts.filter((posts) => posts.id !== id)),
};

export default {
  state,
  getters,
  actions,
  mutations,
};
