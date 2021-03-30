<template>
  <div class="box is-dark post mb-4">
    <article class="media">
      <div class="media-content">
        <div class="content">
          <p>
            <strong>{{ post.author }}</strong>
            <small> ({{ new Date(post.timestamp).toLocaleString() }})</small>
            <br />
            <span class="mt-2" style="white-space: pre-wrap">{{
              post.content
            }}</span>
          </p>
        </div>
        <nav class="level is-mobile">
          <div class="level-left">
            <a
              @click="toggleHeart(post.id)"
              class="level-item"
              id="heart-button"
              aria-label="heart"
            >
              <span class="icon is-small">
                <i class="fa fa-heart"></i>
              </span>
              <small class="p-1"> {{ post.hearts }}</small>
            </a>
          </div>
          <div class="level-right">
            <a @click="deletePost(post.id)" class="level-item">
              <a href="#" class="dropdown-item">
                <i class="fa fa-times" aria-hidden="true"></i>
              </a>
            </a>
          </div>
        </nav>
      </div>
    </article>
  </div>
</template>

<script>
import { mapActions } from "vuex";

export default {
  name: "Post",
  props: {
    post: Object,
  },
  methods: {
    ...mapActions(["deletePost", "toggleHeart"]),
  },
};
</script>