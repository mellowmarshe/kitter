<template>
  <div class="register">
    <div class="columns">
      <div class="column is-three-fifths">
        <Register_ />
      </div>
      <div class="column">
        <div class="box">
          <h2>Requirements</h2>
          <br />
          <h4>Username</h4>
          <p>
            Usernames must be between 3 and 32 characters. They can contain 1-9,
            a-Z, . and \_. They must start with a-Z.
          </p>
          <br />
          <h4>Email</h4>
          <p>Only valid name@domain.com emails.</p>
          <br />
          <h4>Password</h4>
          <p>
            Password must be atleast 8 characters with an uppercase, lowercase
            and number. Special characters are allowed.
          </p>
          <br />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Register_ from "../components/Register_.vue";

export default {
  name: "Register",
  components: {
    Register_,
  },
};
</script>
