<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'

import useLocale from '@/composables/useLocale.ts'

import TheHeader from './TheHeader.vue'

const router = useRouter()
const { translate } = useLocale()

const allRoutes = computed(() =>
  router
    .getRoutes()
    .filter(({ meta }) => meta.isNavigationLink && meta.titleLocaleKey)
    .map((route) => {
      route.meta.title = translate(route.meta.titleLocaleKey)
      return route
    })
)
</script>

<template>
  <t-grid id="the-sidebar" rows>
    <the-header />
    <t-list>
      <li role="listitem" v-for="route in allRoutes">
        <router-link :to="route">{{ route.meta.title }}</router-link>
      </li>
    </t-list>
  </t-grid>
</template>

<style scoped>
#the-sidebar {
  grid-template-rows: max-content auto;
  padding: var(--spacing-none) var(--spacing-4);
  border-right: var(--border-default);
  row-gap: var(--spacing-4);
  height: var(--spacing-all);
}
</style>
