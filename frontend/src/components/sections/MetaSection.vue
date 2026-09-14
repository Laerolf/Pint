<script setup lang="ts">
import useLocale from '@/composables/useLocale'

type MetaFieldKey = 'source' | 'sourceId' | 'createdAt' | 'lastUpdatedAt'

export type MetaField = {
  key: MetaFieldKey
  value: string | undefined | null
}

type Props = {
  fields: MetaField[]
}

defineProps<Props>()

const { translateInScope } = useLocale('sections.meta')
</script>

<template>
  <p-grid rows class="p-meta-section">
    <h2>{{ translateInScope('title') }}</h2>

    <p-grid class="p-meta-section-fields">
      <p-meta-field
        v-for="field in fields"
        :meta-key="field.key"
        :meta-value="field.value"
        :key="`meta-${field.key}`"
      />
    </p-grid>
  </p-grid>
</template>

<style scoped>
.p-meta-section {
  row-gap: var(--spacing-2);
  margin-top: var(--spacing-12);
  padding-top: var(--spacing-6);
  border-top: var(--border-subtle);

  h2 {
    font-size: var(--spacing-4);
    color: var(--text-muted);
  }
}

.p-meta-section-fields {
  column-gap: var(--spacing-4);
}
</style>
