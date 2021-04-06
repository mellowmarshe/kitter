<template>
  <button class="button is-black" @click="onClick()">
    {{ text }}
  </button>
</template>

<script>
export default {
  name: "Button",
  props: {
    text: { type: String, default: "" },
  },
  methods: {
    onClick() {
      console.log("owo");
    },
  },
};
</script>

<style lang="scss" scoped></style>
