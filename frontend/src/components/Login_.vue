<template>
  <div id="errors"></div>
  <form @submit="onSubmit" class="box mt-15" id="login-form">
    <div class="field">
      <label class="label">Username</label>
      <div class="control">
        <input
          class="input"
          v-model="username"
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
          class="input"
          v-model="password"
          name="password"
          type="password"
          placeholder=""
        />
      </div>
    </div>
    <input class="button is-black" type="submit" value="Login" />
  </form>
</template>

<script>
import { mapActions } from "vuex";

export default {
  name: "Login_",
  components: {},
  methods: {
    ...mapActions(["fetchToken"]),
    async onSubmit(e) {
      e.preventDefault();

      const login = {
        username: this.username,
        password: this.password,
      };

      this.fetchToken(login).then(() => {
        console.log(this.$store.state);

        this.$router.push("/");
      });
    },
  },
};
</script>

<style lang="scss" scoped></style>