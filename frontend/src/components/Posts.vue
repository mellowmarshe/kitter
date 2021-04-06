<template>
  <div>
    <div v-for="post in getPosts" :key="post.id">
      <Post :post="post" />
    </div>
  </div>
</template>

<script>
import { mapActions, mapGetters } from "vuex";

import Post from "./Post";

export default {
  name: "Posts",
  components: {
    Post,
  },
  data() {
    return {
      posts: [],
    };
  },
  computed: mapGetters(["getPosts"]),
  created() {
    this.fetchPosts();
  },
  methods: {
    ...mapActions(["fetchPosts"]),
    async addPost(post) {
      this.posts = [...this.posts, post];
    },
  },
};
</script>
