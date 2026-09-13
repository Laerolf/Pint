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
  <t-grid rows class="t-input">
    <t-grid rows class="t-input-content">
      <label class="t-input-label" v-if="$slots.label" :for="name">
        <slot name="label" />
      </label>

      <t-grid class="t-input-fields">
        <slot />
        <slot name="append" />
      </t-grid>
    </t-grid>

    <p class="t-input-error" :class="errorMessageClasses">
      <slot name="error" />
    </p>
  </t-grid>
</template>

<style scoped>
.t-input {
  gap: var(--spacing-none);

  .t-input-content {
    gap: var(--spacing-1);
  }

  .t-input-label {
    font-size: var(--font-size-3);
    font-weight: var(--font-weight-2);
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: var(--letter-spacing-2);
  }

  .t-input-fields {
    column-gap: var(--spacing-4);
    grid-template-columns: auto max-content;

    &:has(> *:only-child) {
      grid-template-columns: auto;
    }
  }

  .t-input-error {
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
