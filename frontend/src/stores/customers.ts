import { defineStore } from 'pinia'
import { computed, inject, reactive, ref } from 'vue'

import { getAllCustomers } from '@/api/sdk.gen'
import { apiClientKey } from '@/plugins/api'

import type { CustomerDto } from '@/api'
import type { ActionState } from '@/shared/store'

/**
 * Represents a store for Customers.
 */
export const useCustomersStore = defineStore('customers', () => {
  const client = inject(apiClientKey)

  const customers = ref<CustomerDto[]>([])

  const fetchAllState = reactive<ActionState>({ isLoading: false })

  const allCustomers = computed<CustomerDto[]>(() => customers.value)

  /**
   * Fetch all Customers from the API.
   */
  async function fetchAll(): Promise<void> {
    fetchAllState.isLoading = true

    try {
      const response = await getAllCustomers({ client })
      customers.value = response.data || []
    } catch (error) {
      throw new Error(`Failed to fetch all Customers from the API: ${(error as Error).message}`, {
        cause: error
      })
    } finally {
      fetchAllState.isLoading = false
    }
  }

  return {
    allCustomers,
    fetchAllState,
    fetchAll
  }
})
