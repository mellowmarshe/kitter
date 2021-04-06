<template>
  <div>
    <div v-for="(error, index) in getErrors" :key="index">
      <Error :error="error" :index="index" />
    </div>
  </div>
</template>

<script>
import { mapGetters } from "vuex";

import Error from "./Error";

export default {
  name: "Errors",
  components: {
    Error,
  },
  data() {
    return {
      errors: [],
    };
  },
  computed: mapGetters(["getErrors"]),
  methods: {
    async addError(error) {
      this.errors = [...this.errors, error];
    },
  },
};
</script>
