import { fileURLToPath, URL } from 'node:url'

import { heyApiPlugin } from '@hey-api/vite-plugin'
import vue from '@vitejs/plugin-vue'
import Components from 'unplugin-vue-components/vite'
import { defineConfig, loadEnv } from 'vite'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig(async ({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')

  const OPEN_API_JSON_URL = `${env.VITE_BASE_API_URL}/openapi.json`

  const isApiAvailable =
    mode !== 'test' &&
    (await fetch(OPEN_API_JSON_URL)
      .then(() => true)
      .catch(() => false))

  return {
    plugins: [
      vue(),
      vueDevTools(),
      isApiAvailable &&
        heyApiPlugin({
          config: {
            input: OPEN_API_JSON_URL,
            output: 'src/api',
            logs: 'logs/openapi'
          }
        }),
      Components({
        dirs: ['src/components/global']
      })
    ],
    server: {
      host: env.VITE_HOST || 'localhost',
      port: Number(env.VITE_PORT) || 5173
    },
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
      }
    }
  }
})
