<template>
  <form class="p-form">
    <p-grid class="content" rows>
      <p-grid class="fields" rows>
        <slot />
      </p-grid>

      <p-grid v-if="$slots.actions" class="actions">
        <slot name="actions" />
      </p-grid>
    </p-grid>
  </form>
</template>

<style scoped>
.p-form {
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
