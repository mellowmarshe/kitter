<template>
  <div v-if="isShown" class="modal is-justify-content-center is-flex" hidden>
    <div class="modal-background"></div>
    <div class="modal-content mt-15">
      <form @submit="onSubmit" class="box mt-15" id="post-form">
        <div class="field">
          <label class="label">Content</label>
          <div class="control">
            <textarea
              class="textarea"
              v-model="content"
              id="post-content"
              name="content"
              placeholder="Post content; up to 512 characters."
            ></textarea>
          </div>
        </div>
        <input class="button is-black" type="submit" value="Post" />
        <button
          @click="toggleShown"
          class="modal-close is-large"
          id="post-close"
          aria-label="close"
        ></button>
      </form>
    </div>
  </div>
  <button
    @click="toggleShown"
    class="button is-black ml-4"
    value="post"
    id="post-button"
    href="#"
  >
    <i class="fa fa-plus"></i>
  </button>
</template>

<script>
import { mapActions } from "vuex";

export default {
  name: "NewPostModal",
  data() {
    return {
      isShown: false,
    };
  },
  components: {},
  methods: {
    ...mapActions(["addPost"]),
    async onSubmit(e) {
      e.preventDefault();

      const post = {
        content: this.content,
      };

      this.content = "";

      this.addPost(post);

      this.toggleShown();
    },

    async toggleShown() {
      this.isShown = !this.isShown;
    },
  },
};
</script>