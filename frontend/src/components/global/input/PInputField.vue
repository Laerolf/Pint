<script setup lang="ts">
import { useField } from 'vee-validate'
import { computed } from 'vue'

type Props = {
  /**
   * The name of the input field.
   */
  name: string
  /**
   * Should the input field value be masked?
   */
  secret?: boolean
  readonly?: boolean
}

const props = defineProps<Props>()

const { value, errorMessage, handleBlur } = useField(() => props.name)

const type = computed(() => (props.secret ? 'password' : 'text'))

const classes = computed(() => ({ error: !!errorMessage.value }))
</script>

<template>
  <p-input :name="name">
    <template #label>
      <slot />
    </template>

    <input
      v-if="!readonly"
      v-bind="$attrs"
      class="p-input-field"
      :class="classes"
      :id="name"
      :name="name"
      v-model="value"
      @blur="handleBlur"
      :type="type"
    />
    <p v-else>{{ value }}</p>

    <template #append>
      <slot name="append" />
    </template>

    <template #error>{{ errorMessage }}</template>
  </p-input>
</template>

<style scoped>
.p-input-field {
  width: var(--spacing-all);
  border: var(--border-default);

  &.error {
    border-color: var(--color-error);
  }
}
</style>
