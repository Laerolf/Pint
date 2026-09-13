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
  <t-grid rows class="t-meta-field">
    <label class="t-meta-field-label">
      {{ translateInScope(metaKey) }}
    </label>

    <t-grid class="t-meta-field-value">
      <p>{{ metaValue ?? '-' }}</p>
    </t-grid>
  </t-grid>
</template>

<style scoped>
.t-meta-field {
  gap: var(--spacing-1);

  .t-meta-field-label {
    font-size: var(--font-size-3);
    font-weight: var(--font-weight-2);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: var(--letter-spacing-2);
  }

  .t-meta-field-value {
    font-family: var(--font-mono);
    font-size: var(--font-size-3);
    color: var(--text-muted);
  }
}
</style>
