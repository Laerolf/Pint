<template>
  <form class="t-form">
    <t-grid class="content" rows>
      <t-grid class="fields" rows>
        <slot />
      </t-grid>

      <t-grid v-if="$slots.actions" class="actions">
        <slot name="actions" />
      </t-grid>
    </t-grid>
  </form>
</template>

<style scoped>
.t-form {
  width: var(--spacing-all);

  .content {
    row-gap: var(--spacing-4);

    .fields {
      row-gap: var(--spacing-2);
    }

    .actions {
      grid-auto-columns: max-content;
      align-items: center;
      column-gap: var(--spacing-2);
    }
  }
}
</style>
