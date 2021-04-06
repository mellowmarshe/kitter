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
              id="heart-button"
              class="level-item"
              aria-label="heart"
              @click="toggleHeart(post.id)"
            >
              <span class="icon is-small">
                <i
                  :class="[
                    post.hearted_users.includes($store.state.user.id)
                      ? 'has-text-danger'
                      : '',
                    'has-text-black',
                  ]"
                  class="fa fa-heart"
                />
              </span>
              <small class="p-1"> {{ post.hearts }}</small>
            </a>
          </div>
          <div class="level-right">
            <a
              v-if="post.author_id == $store.state.user.id"
              class="level-item"
              @click="deletePost(post.id)"
            >
              <a href="#" class="dropdown-item">
                <i class="fa fa-times" aria-hidden="true" />
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
    post: { type: Object, default: null },
  },
  methods: {
    ...mapActions(["deletePost", "toggleHeart"]),
  },
};
</script>
