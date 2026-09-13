import { createClient } from '@/api/client'
import useEnv from '@/composables/useEnvironment'

import type { Client } from '@/api/client'
import type { InjectionKey, Plugin } from 'vue'

export const apiClientKey: InjectionKey<Client> = Symbol('apiClient')

export default {
  install(app) {
    const { apiUrl } = useEnv()

    const client = createClient({ baseUrl: apiUrl })

    app.provide(apiClientKey, client)
  }
} satisfies Plugin
