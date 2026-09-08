<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { computed, onMounted } from 'vue'

import useLocale from '@/composables/useLocale'
import { useCustomersStore } from '@/stores/customers'

import type { TTableHeader } from '@/components/global/structure/TTable.vue'

const customersStore = useCustomersStore()
const { allCustomers, fetchAllState } = storeToRefs(customersStore)

const { translateInScope } = useLocale('pages.customers.overview')

const customersTableHeaders = computed<TTableHeader[]>(() => [
  { label: translateInScope('table.headers.lastName'), key: 'last_name' },
  { label: translateInScope('table.headers.firstName'), key: 'first_name' },
  { label: translateInScope('table.headers.nickname'), key: 'nickname' }
])

onMounted(async () => {
  try {
    await customersStore.fetchAll()
  } catch (error) {
    console.error(`Failed to load the Customers overview page: ${(error as Error).message}`)
  }
})
</script>

<template>
  <t-page>
    <t-table
      class="customers-table"
      :headers="customersTableHeaders"
      :items="allCustomers"
      :loading="fetchAllState.isLoading"
    />
  </t-page>
</template>
