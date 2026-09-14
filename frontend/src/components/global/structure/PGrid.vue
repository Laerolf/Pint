<script setup lang="ts">
import { computed } from 'vue'

type Props = {
  /**
   * The child items should be displayed in rows.
   */
  rows?: boolean
}

const props = defineProps<Props>()

const classes = computed(() => ({
  rows: props.rows
}))
</script>

<template>
  <div class="p-grid" :class="classes">
    <slot />
  </div>
</template>

<style scoped>
.p-grid {
  display: grid;
  /* height: var(--spacing-all); */
  width: var(--spacing-all);
  grid-auto-flow: column;
  place-items: start;

  &.rows {
    grid-auto-flow: row;
  }
}
</style>
