<script setup lang="ts">
import { computed } from 'vue'

export type TTableHeader = {
  /**
   * The label to display in the header for this column.
   */
  label: string
  /**
   * The item property to display in an item row for this column.
   */
  key: string
}

type Props = {
  /**
   * The headers to display.
   */
  headers: TTableHeader[]
  /**
   * The items to display in rows.
   */
  items: Record<string, unknown>[]
  /**
   * The table is currently waiting for its data to be loaded.
   */
  loading?: boolean
  /**
   * The amount of skeleton rows to show.
   */
  skeletonRowsToShow?: number
  /**
   * The table rows are clickable.
   */
  clickable?: boolean
}

const props = withDefaults(defineProps<Props>(), { skeletonRowsToShow: 10 })

const emit = defineEmits<{ (event: 'row:select', index: number): void }>()

const rowClasses = computed(() => ({
  clickable: props.clickable
}))

function handleRowClick(index: number) {
  emit('row:select', index)
}
</script>

<template>
  <table class="t-table">
    <thead>
      <tr>
        <th v-for="header in headers">{{ header.label }}</th>
      </tr>
    </thead>

    <tbody v-if="loading">
      <tr v-for="_skeletonRow in skeletonRowsToShow">
        <td v-for="_header in headers">
          <div class="skeleton" />
        </td>
      </tr>
    </tbody>

    <tbody v-else>
      <tr @click="handleRowClick(index)" v-for="(item, index) in items" :class="rowClasses">
        <td v-for="header in headers">
          <slot :name="`item-${header.key}`" v-bind="{ key: header.key, item }">
            {{ item[header.key] }}
          </slot>
        </td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
.t-table {
  border: var(--border-default);
  border-collapse: collapse;
  width: var(--spacing-all);

  thead {
    border-bottom: var(--border-default);
  }

  tbody tr {
    &.clickable {
      cursor: pointer;
    }
  }

  tbody tr:nth-child(even) {
    background-color: var(--color-accent);
  }

  th,
  td {
    padding: var(--spacing-none) var(--spacing-3);
  }

  td {
    text-align: center;
  }
}

.skeleton {
  height: 1lh;
  width: var(--spacing-all);
  background-color: var(--color-muted);
}
</style>
