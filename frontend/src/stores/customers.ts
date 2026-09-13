import { defineStore } from 'pinia'
import { inject, reactive, ref } from 'vue'

import { findCustomerById, getAllCustomers } from '@/api/sdk.gen'
import { apiClientKey } from '@/plugins/api'
import { FetchActionState } from '@/shared/store'

import type { CustomerDto } from '@/api'

/**
 * Represents a store for Customers.
 */
export const useCustomersStore = defineStore('customers', () => {
  const client = inject(apiClientKey)

  const customers = ref<CustomerDto[]>([])

  const fetchAllState = reactive<FetchActionState>(new FetchActionState())
  const findByIdState = reactive<FetchActionState>(new FetchActionState())

  /**
   * Finds a Customer with the provided ID from the API.
   */
  async function findById(id: number): Promise<void> {
    if (customers.value.some((customer) => customer.id === id)) {
      return
    }

    findByIdState.loading = true

    try {
      const response = await findCustomerById({ client, path: { id } })

      if (response.data) {
        customers.value.push(response.data)
      }
    } catch (error) {
      findByIdState.error = error
      throw new Error(
        `Failed to find a Customer with the provided ID from the API: ${(error as Error).message}`,
        {
          cause: error
        }
      )
    } finally {
      findByIdState.loading = false
      findByIdState.lastRunAt = new Date()
    }
  }

  /**
   * Fetch all Customers from the API.
   */
  async function fetchAll(): Promise<void> {
    if (!fetchAllState.isStale()) {
      return
    }

    fetchAllState.loading = true

    try {
      const response = await getAllCustomers({ client })
      customers.value = response.data || []
    } catch (error) {
      fetchAllState.error = error
      throw new Error(`Failed to fetch all Customers from the API: ${(error as Error).message}`, {
        cause: error
      })
    } finally {
      fetchAllState.loading = false
      fetchAllState.lastRunAt = new Date()
    }
  }

  return {
    customers,
    findByIdState,
    fetchAllState,
    findById,
    fetchAll
  }
})
