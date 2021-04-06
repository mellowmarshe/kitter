<template>
  <div class="notification is-black mb-4">
    <button @click="deleteError(index)" class="delete" />
    <template v-if="error.response">
      <span class="tag is-danger">{{ error.response.data.status_code }}</span>
      {{ error.response.data.error }}
    </template>
    <template v-else>
      <span class="tag is-danger">Unknown</span>
      Unknown error occurred.
    </template>
  </div>
</template>

<script>
import { mapActions } from "vuex";

export default {
  name: "Error",
  props: {
    error: { type: Object, default: null },
    index: { type: Number, default: NaN },
  },
  methods: {
    ...mapActions(["deleteError"]),
  },
};
</script>
