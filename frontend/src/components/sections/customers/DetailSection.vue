<script setup lang="ts">
import useLocale from '@/composables/useLocale'

const { translateInScope } = useLocale('sections.customers.detail')
</script>

<template>
  <p-form @submit.prevent>
    <p-grid rows class="p-form-section">
      <h2>{{ translateInScope('sections.about.title') }}</h2>

      <p-grid class="p-form-section-fields">
        <p-input-field name="firstName" readonly>{{
          translateInScope('sections.about.fields.firstName')
        }}</p-input-field>

        <p-input-field name="lastName" readonly>
          {{ translateInScope('sections.about.fields.lastName') }}
        </p-input-field>

        <p-input-field name="nickname" readonly>
          {{ translateInScope('sections.about.fields.nickname') }}
        </p-input-field>

        <p-input-field name="dateOfBirth" readonly>
          {{ translateInScope('sections.about.fields.dateOfBirth') }}
        </p-input-field>
      </p-grid>
    </p-grid>

    <p-grid rows class="p-form-section">
      <h2>{{ translateInScope('sections.contact.title') }}</h2>

      <p-grid class="p-form-section-fields">
        <p-input-field name="emailAddress" readonly>
          {{ translateInScope('sections.contact.fields.emailAddress') }}
        </p-input-field>
      </p-grid>
    </p-grid>
  </p-form>
</template>

<style scoped>
.p-form-section {
  row-gap: var(--spacing-2);
}

.p-form-section-fields {
  column-gap: var(--spacing-4);
}
</style>
