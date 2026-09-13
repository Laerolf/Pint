<script setup lang="ts">
import useLocale from '@/composables/useLocale'

const { translateInScope } = useLocale('sections.customers.detail')
</script>

<template>
  <t-form @submit.prevent>
    <t-grid rows class="t-form-section">
      <h2>{{ translateInScope('sections.about.title') }}</h2>

      <t-grid class="t-form-section-fields">
        <t-input-field name="firstName" readonly>{{
          translateInScope('sections.about.fields.firstName')
        }}</t-input-field>

        <t-input-field name="lastName" readonly>
          {{ translateInScope('sections.about.fields.lastName') }}
        </t-input-field>

        <t-input-field name="nickname" readonly>
          {{ translateInScope('sections.about.fields.nickname') }}
        </t-input-field>

        <t-input-field name="dateOfBirth" readonly>
          {{ translateInScope('sections.about.fields.dateOfBirth') }}
        </t-input-field>
      </t-grid>
    </t-grid>

    <t-grid rows class="t-form-section">
      <h2>{{ translateInScope('sections.contact.title') }}</h2>

      <t-grid class="t-form-section-fields">
        <t-input-field name="emailAddress" readonly>
          {{ translateInScope('sections.contact.fields.emailAddress') }}
        </t-input-field>
      </t-grid>
    </t-grid>
  </t-form>
</template>

<style scoped>
.t-form-section {
  row-gap: var(--spacing-2);
}

.t-form-section-fields {
  column-gap: var(--spacing-4);
}
</style>
