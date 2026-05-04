import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import ConfigView from '../views/ConfigView.vue'
import DevToolsView from '../views/DevToolsView.vue'
import SettingsView from '../views/SettingsView.vue'

const LAST_ROUTE_KEY = 'nova-deck:last-route'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'config',
        component: ConfigView
    },
    {
        path: '/dev',
        name: 'dev',
        component: DevToolsView
    },
    {
        path: '/settings',
        name: 'settings',
        component: SettingsView
    }
]

const restorablePaths = new Set(routes.map((route) => route.path))

export function getRestorableRoute(): string | null {
    if (typeof window === 'undefined') {
        return null
    }

    const lastRoute = window.localStorage.getItem(LAST_ROUTE_KEY)
    if (!lastRoute || !restorablePaths.has(lastRoute)) {
        return null
    }

    return lastRoute
}

const router = createRouter({
    history: createWebHistory(),
    routes
})

router.afterEach((to) => {
    if (typeof window === 'undefined') {
        return
    }

    window.localStorage.setItem(LAST_ROUTE_KEY, to.path)
})

export default router
