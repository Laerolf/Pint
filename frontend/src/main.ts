import { createHead } from '@unhead/vue/client'
import { createPinia } from 'pinia'
import { createApp } from 'vue'

import App from '@/App.vue'
import apiClient from '@/plugins/api'
import '@/assets/styling/main.css'
import i18n from '@/plugins/i18n'
import router from '@/router'

const head = createHead()

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.use(head)
app.use(apiClient)
app.use(i18n)

app.mount('#app')
