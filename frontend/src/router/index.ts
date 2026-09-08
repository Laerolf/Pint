import { createRouter, createWebHistory } from 'vue-router'

import { localeGuard, pageTitleGuard } from '@/middleware/guard'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: () => import('@/layouts/DefaultLayout.vue'),
      children: [
        {
          path: '',
          redirect: { name: 'customersOverview' }
        },
        {
          path: 'customers',
          children: [
            {
              name: 'customersOverview',
              path: '',
              component: () => import('@/pages/customers/OverviewPage.vue'),
              meta: { titleLocaleKey: 'pages.customers.overview.title', isNavigationLink: true }
            }
          ]
        }
      ]
    }
  ]
})

router.beforeEach(localeGuard)
router.beforeEach(pageTitleGuard)

export default router
