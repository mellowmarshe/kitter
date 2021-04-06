<template>
  <div class="home">
    <div class="columns">
      <div class="column is-three-fifths pr-0">
        <Posts />
      </div>
      <div class="column ml-4 pl-0">
        <div>
          <button class="button is-black">
            <strong>{{ $store.state.user.username }}</strong>
          </button>
          <NewPostModal />
        </div>

        <div class="box mt-4">
          <strong>More to come here!</strong>
          <br />
          <strong>Maybe tags....</strong>
          <br />
          <strong>Or sneak peaks at features....</strong>
          <br />
          <strong>Such as users most recent post...</strong>
        </div>
        <Errors />
      </div>
    </div>
  </div>
</template>

<script>
import Posts from "../components/Posts.vue";
import NewPostModal from "../components/NewPostModal.vue";
import Errors from "../components/Errors.vue";

import { mapActions } from "vuex";

export default {
  name: "Home",
  components: {
    Posts,
    NewPostModal,
    Errors,
  },
  created() {
    this.fetchUser();
  },
  methods: {
    ...mapActions(["fetchUser"]),
  },
};
</script>
