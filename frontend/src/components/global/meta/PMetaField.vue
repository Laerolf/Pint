<script setup lang="ts">
import useLocale from '@/composables/useLocale'

type Props = {
  metaKey: string
  metaValue?: string
}

defineProps<Props>()

const { translateInScope } = useLocale('sections.meta.fields')
</script>

<template>
  <p-grid rows class="p-meta-field">
    <label class="p-meta-field-label">
      {{ translateInScope(metaKey) }}
    </label>

    <p-grid class="p-meta-field-value">
      <p>{{ metaValue ?? '-' }}</p>
    </p-grid>
  </p-grid>
</template>

<style scoped>
.p-meta-field {
  gap: var(--spacing-1);

  .p-meta-field-label {
    font-size: var(--font-size-3);
    font-weight: var(--font-weight-2);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: var(--letter-spacing-2);
  }

  .p-meta-field-value {
    font-family: var(--font-mono);
    font-size: var(--font-size-3);
    color: var(--text-muted);
  }
}
</style>
