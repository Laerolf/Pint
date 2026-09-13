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
  <t-input :name="name">
    <template #label>
      <slot />
    </template>

    <input
      v-if="!readonly"
      v-bind="$attrs"
      class="t-input-field"
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
  </t-input>
</template>

<style scoped>
.t-input-field {
  width: var(--spacing-all);
  border: var(--border-default);

  &.error {
    border-color: var(--color-error);
  }
}
</style>
