/// <reference types="vite/client" />

export {}

export type EnvVarKey = keyof ImportMetaEnv

interface ViteTypeOptions {
  // By adding this line, you can make the type of ImportMetaEnv strict
  // to disallow unknown keys.
}

interface ImportMetaEnv {
  /**
   * The API base url to connect to.
   */
  readonly VITE_BASE_API_URL: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

declare module 'vue-router' {
  interface RouteMeta {
    /**
     * The i18n locale key of the route's title.
     */
    titleLocaleKey: string
    /**
     * The title of the route.
     */
    title?: string
    /**
     * Display the route in the navigation.
     */
    isNavigationLink?: boolean
  }
}
