<template>
  <div>
    <Errors />
    <form id="login-form" class="box mt-15" @submit="onSubmit">
      <div class="field">
        <label class="label">Username</label>
        <div class="control">
          <input
            v-model="username"
            class="input"
            name="username"
            type="text"
            placeholder=""
          />
        </div>
      </div>
      <div class="field">
        <label class="label">Password</label>
        <div class="control">
          <input
            v-model="password"
            class="input"
            name="password"
            type="password"
            placeholder=""
          />
        </div>
      </div>
      <input class="button is-black" type="submit" value="Login" />
    </form>
  </div>
</template>

<script>
import { mapActions } from "vuex";

import Errors from "./Errors.vue";

export default {
  // eslint-disable-next-line vue/component-definition-name-casing
  name: "Login_",
  components: {
    Errors,
  },
  methods: {
    ...mapActions(["fetchToken", "addError", "deleteAllErrors"]),
    async onSubmit(e) {
      e.preventDefault();

      const login = {
        username: this.username,
        password: this.password,
      };

      this.fetchToken(login).then(
        () => {
          this.deleteAllErrors();
          this.$router.push("/");
        },
        (err) => {
          this.addError(err);
        }
      );
    },
  },
};
</script>

<style lang="scss" scoped></style>
