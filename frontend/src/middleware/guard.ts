import { i18n, loadLocaleMessages } from '@/plugins/i18n'

import type { SupportedLocale } from '@/plugins/i18n'
import type { NavigationGuard } from 'vue-router'

/**
 * A navigation guard loading the i18n locale message.
 */
export const localeGuard: NavigationGuard = async () => {
  try {
    await loadLocaleMessages(i18n.global.locale.value as SupportedLocale)
  } catch (error) {
    console.error(
      `Failed to load the '${i18n.global.locale.value}' i18n locale messages => ${(error as Error).message}`,
      {
        cause: error
      }
    )
  }
}

/**
 * A navigation guard translating the page meta title.
 */
export const pageTitleGuard: NavigationGuard = (to) => {
  try {
    if (to.meta.titleLocaleKey) {
      to.meta.title = i18n.global.t(to.meta.titleLocaleKey)
    }
  } catch (error) {
    console.error(`Failed to translate the page title => ${(error as Error).message}`, {
      cause: error
    })
  }
}
