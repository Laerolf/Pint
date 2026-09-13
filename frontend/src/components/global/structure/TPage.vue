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
  <t-grid rows class="t-page">
    <t-grid class="t-page-header" rows>
      <h1 class="t-page-title">{{ title }}</h1>

      <t-grid v-if="$slots.actions" class="t-page-actions">
        <slot name="actions" />
      </t-grid>
    </t-grid>

    <t-grid class="t-page-content">
      <slot />
    </t-grid>
  </t-grid>
</template>

<style scoped>
.t-page {
  grid-template-rows: repeat(2, max-content);
  padding: var(--spacing-none) var(--spacing-4);
  row-gap: var(--spacing-8);
  margin-bottom: var(--spacing-4);

  .t-page-header {
    row-gap: var(--spacing-4);
  }
}
</style>
