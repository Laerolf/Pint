<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

type Props = {
  title?: string
}

const props = defineProps<Props>()

const route = useRoute()

const title = computed<string | undefined>(() => props.title || route.meta.title)
</script>

<template>
  <p-grid rows class="p-page">
    <p-grid class="p-page-header" rows>
      <h1 class="p-page-title">{{ title }}</h1>

      <p-grid v-if="$slots.actions" class="p-page-actions">
        <slot name="actions" />
      </p-grid>
    </p-grid>

    <p-grid class="p-page-content">
      <slot />
    </p-grid>
  </p-grid>
</template>

<style scoped>
.p-page {
  grid-template-rows: repeat(2, max-content);
  padding: var(--spacing-none) var(--spacing-4);
  row-gap: var(--spacing-8);
  margin-bottom: var(--spacing-4);

  .p-page-header {
    row-gap: var(--spacing-4);
  }
}
</style>
