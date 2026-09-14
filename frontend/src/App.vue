<script setup lang="ts">
import { useHead } from '@unhead/vue'
import { useRoute } from 'vue-router'

const route = useRoute()

useHead({
  titleTemplate: '%s | Pint',
  title: () => route.meta.title
})
</script>

<template>
  <router-view />
</template>
