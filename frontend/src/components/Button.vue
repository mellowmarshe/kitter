<template>
  <button @click="onClick()" class="button is-black">{{ text }}</button>
</template>

<script>
export default {
  name: "Button",
  props: {
    text: String,
  },
  methods: {
    onClick() {
      console.log("owo");
    },
  },
};
</script>

<style lang="scss" scoped></style>
