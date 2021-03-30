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
import axios from "axios";

export default {
  name: "Login_",
  components: {},
  methods: {
    async onSubmit(e) {
      e.preventDefault();

      const login = {
        username: this.username,
        password: this.password,
      };

      const res = await axios.post(
        "http://localhost:8083/api/user/login",
        JSON.stringify(login),
        {
          headers: {
            "Content-type": "application/json",
          },
        }
      );
      const data = res.data;
      this.$store.state.token = data["token"];
      this.$store.state.username = login.username;
      this.$store.state.id = login.id;

      this.$router.push("/");
    },
  },
};
</script>

<style lang="scss" scoped></style>