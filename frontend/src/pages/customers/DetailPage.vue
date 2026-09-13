<script setup lang="ts">
import { useHead } from '@unhead/vue'
import { toTypedSchema } from '@vee-validate/zod'
import { storeToRefs } from 'pinia'
import { useForm } from 'vee-validate'
import { computed, ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { object, string } from 'zod'

import DetailSection from '@/components/sections/customers/DetailSection.vue'
import MetaSection, { type MetaField } from '@/components/sections/MetaSection.vue'
import { useCustomersStore } from '@/stores/customers'

import type { CustomerDto } from '@/api'

const head = useHead()
const route = useRoute()

const customersStore = useCustomersStore()
const { customers, fetchAllState } = storeToRefs(customersStore)

const schema = object({
  firstName: string().nullish(),
  lastName: string().nullish(),
  nickname: string().nullish(),
  dateOfBirth: string().nullish(),
  emailAddress: string().nullish()
})

const selectedCustomer = ref<CustomerDto | undefined>()

const typedSchema = computed(() => toTypedSchema(schema))

const customerFullName = computed<string | undefined>(() =>
  [selectedCustomer.value?.last_name || '?', selectedCustomer.value?.first_name || '?']
    .filter((part) => part)
    .join(' ')
)

const metaFields = computed<MetaField[]>(() => [
  { key: 'source', value: selectedCustomer.value?.source },
  { key: 'sourceId', value: selectedCustomer.value?.source_id },
  { key: 'createdAt', value: selectedCustomer.value?.created_at },
  { key: 'lastUpdatedAt', value: selectedCustomer.value?.last_updated_at }
])

const title = computed<string>(() => customerFullName.value ?? '??')

const form = useForm({
  validationSchema: typedSchema.value
})

onMounted(async () => {
  try {
    await customersStore.fetchAll()

    let optionalSelectedCustomer = customers.value.find(
      ({ id }) => id.toString() === route.params.id
    )

    if (!optionalSelectedCustomer) {
      throw new Error(`Failed to find the selected Customer with ID '${route.params.id}'.`)
    }

    selectedCustomer.value = optionalSelectedCustomer

    head.patch({ title: title.value })
    form.setValues({
      firstName: selectedCustomer.value?.first_name,
      lastName: selectedCustomer.value?.last_name,
      nickname: selectedCustomer.value?.nickname,
      dateOfBirth: selectedCustomer.value?.date_of_birth,
      emailAddress: selectedCustomer.value?.email_address
    })
  } catch (error) {
    console.error(`Failed to load the Customers detail page: ${(error as Error).message}`)
  }
})
</script>

<template>
  <t-page :title="title">
    <t-grid rows>
      <detail-section />
      <meta-section :fields="metaFields" />
    </t-grid>
  </t-page>
</template>
