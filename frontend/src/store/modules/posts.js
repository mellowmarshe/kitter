import axios from "axios";
import config from "../../config";

const state = {
  posts: [],
};

const getters = {
  getPosts: (state) => state.posts,
};

const actions = {
  async fetchPosts({ commit, rootState }) {
    const res = await axios.post(
      `${config.url}/api/post/posts`,
      JSON.stringify({ offset: 0, limit: 25 }),
      {
        headers: {
          "Content-type": "application/json",
          Authorization: `Bearer ${rootState.auth.token}`,
        },
      }
    );
    commit("setPosts", res.data);
  },
  async addPost({ commit, rootState }, post) {
    const res = await axios.post(
      `${config.url}/api/post/add`,
      JSON.stringify(post),
      {
        headers: {
          "Content-type": "application/json",
          Authorization: `Bearer ${rootState.auth.token}`,
        },
      }
    );

    commit("newPost", res.data);
  },
  async deletePost({ commit, rootState }, id) {
    const res = await axios.delete(`${config.url}/api/post/delete`, {
      headers: {
        "Content-type": "application/json",
        Authorization: `Bearer ${rootState.auth.token}`,
      },
      data: JSON.stringify({ id: id }),
    });

    commit("removePost", res.data.id);
  },
  async toggleHeart({ commit, rootState }, id) {
    const res = await axios.post(
      `${config.url}/api/post/heart`,
      JSON.stringify({ id: id }),
      {
        headers: {
          "Content-type": "application/json",
          Authorization: `Bearer ${rootState.auth.token}`,
        },
      }
    );

    commit("updatePost", res.data);
  },
};

const mutations = {
  setPosts: (state, posts) => (state.posts = posts),
  newPost: (state, posts) => state.posts.unshift(posts),
  removePost: (state, id) =>
    (state.posts = state.posts.filter((posts) => posts.id !== id)),
  updatePost: (state, post) => {
    const index = state.posts.findIndex((p) => p.id === post.id);
    if (index !== -1) {
      state.posts.splice(index, 1, post);
    }
  },
};

export default {
  state,
  getters,
  actions,
  mutations,
};
