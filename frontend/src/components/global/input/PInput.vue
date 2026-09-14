<script setup lang="ts">
import { computed, useSlots } from 'vue'

type Props = {
  /**
   * The name of the input.
   */
  name: string
}

const props = defineProps<Props>()

const slots = useSlots()

const errorMessageClasses = computed(() => ({
  visible: !!slots.error
}))
</script>

<template>
  <p-grid rows class="p-input">
    <p-grid rows class="p-input-content">
      <label class="p-input-label" v-if="$slots.label" :for="name">
        <slot name="label" />
      </label>

      <p-grid class="p-input-fields">
        <slot />
        <slot name="append" />
      </p-grid>
    </p-grid>

    <p class="p-input-error" :class="errorMessageClasses">
      <slot name="error" />
    </p>
  </p-grid>
</template>

<style scoped>
.p-input {
  gap: var(--spacing-none);

  .p-input-content {
    gap: var(--spacing-1);
  }

  .p-input-label {
    font-size: var(--font-size-3);
    font-weight: var(--font-weight-2);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: var(--letter-spacing-2);
  }

  .p-input-fields {
    column-gap: var(--spacing-4);
    grid-template-columns: auto max-content;

    &:has(> *:only-child) {
      grid-template-columns: auto;
    }
  }

  .p-input-error {
    width: var(--spacing-all);
    color: var(--color-error);
    min-height: 1lh;
    visibility: hidden;

    &.visible {
      visibility: visible;
    }
  }
}
</style>
