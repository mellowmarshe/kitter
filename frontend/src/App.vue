<template>
  <div id="nav">
    <router-link to="/">Home</router-link> |
    <router-link to="/about">About</router-link> |
    <router-link to="/login">Login</router-link> |
    <router-link to="/register">Register</router-link>
  </div>
  <div class="container content">
    <router-view />
  </div>
</template>


<style lang="scss">
html {
  scroll-behavior: smooth;
}

body {
  font-family: "Roboto", sans-serif;
}
</style>