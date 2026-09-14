<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

import useLocale from '@/composables/useLocale'
import { useCustomersStore } from '@/stores/customers'

import type { CustomerDto } from '@/api'
import type { TTableHeader } from '@/components/global/structure/TTable.vue'

const router = useRouter()

const customersStore = useCustomersStore()
const { customers, fetchAllState } = storeToRefs(customersStore)

const { translateInScope } = useLocale('pages.customers.overview')

const customersTableHeaders = computed<TTableHeader[]>(() => [
  { label: translateInScope('table.headers.lastName'), key: 'last_name' },
  { label: translateInScope('table.headers.firstName'), key: 'first_name' },
  { label: translateInScope('table.headers.nickname'), key: 'nickname' }
])

async function handleCustomerSelect(index: number): Promise<void> {
  const selectedCustomer = customers.value[index]

  if (!selectedCustomer) {
    return
  }

  await router.push({ name: 'customersDetail', params: { id: selectedCustomer.id } })
}

onMounted(async () => {
  try {
    await customersStore.fetchAll()
  } catch (error) {
    console.error(`Failed to load the Customers overview page: ${(error as Error).message}`)
  }
})
</script>

<template>
  <p-page>
    <p-table
      id="customers-table"
      :headers="customersTableHeaders"
      :items="customers"
      :loading="fetchAllState.isLoading"
      clickable
      @row:select="handleCustomerSelect"
    />
  </p-page>
</template>
