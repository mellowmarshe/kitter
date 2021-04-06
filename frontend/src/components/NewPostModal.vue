<template>
  <div v-if="isShown" class="modal is-justify-content-center is-flex" hidden>
    <div class="modal-background" />
    <div class="modal-content mt-15">
      <Errors />
      <form id="post-form" class="box mt-15" @submit="onSubmit">
        <div class="field">
          <label class="label">Content</label>
          <div class="control">
            <textarea
              id="post-content"
              v-model="content"
              class="textarea"
              name="content"
              placeholder="Post content; up to 512 characters."
            />
          </div>
        </div>
        <input class="button is-black" type="submit" value="Post" />
        <button
          id="post-close"
          class="modal-close is-large"
          aria-label="close"
          @click="toggleShown"
        />
      </form>
    </div>
  </div>
  <button
    id="post-button"
    class="button is-black ml-4"
    value="post"
    href="#"
    @click="toggleShown"
  >
    <i class="fa fa-plus" />
  </button>
</template>

<script>
import { mapActions } from "vuex";
import Errors from "./Errors.vue";

export default {
  name: "NewPostModal",
  components: { Errors },
  data() {
    return {
      isShown: false,
    };
  },
  methods: {
    ...mapActions(["addPost", "addError", "deleteAllErrors"]),
    async onSubmit(e) {
      e.preventDefault();

      const post = {
        content: this.content,
      };

      this.content = "";

      this.addPost(post).then(
        () => {
          this.toggleShown();
        },
        (err) => {
          this.addError(err);
        }
      );
    },

    async toggleShown() {
      this.deleteAllErrors();
      this.isShown = !this.isShown;
    },
  },
};
</script>
