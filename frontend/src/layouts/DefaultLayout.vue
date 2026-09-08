<script setup lang="ts">
import TheSidebar from '@/components/TheSidebar.vue'
</script>

<template>
  <t-grid id="default-layout">
    <nav>
      <the-sidebar />
    </nav>
    <main><router-view /></main>
  </t-grid>
</template>

<style scoped>
#default-layout {
  width: 100vw;
  height: 100vh;
  grid-template-columns: max-content auto;
  align-items: stretch;
  justify-items: stretch;
}
</style>
