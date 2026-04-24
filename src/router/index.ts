import { createRouter, createWebHistory } from 'vue-router'
import ConfigView from '../views/ConfigView.vue'
import TestView from '../views/TestView.vue'
import SettingsView from '../views/SettingsView.vue'

const routes = [
    {
        path: '/',
        name: 'config',
        component: ConfigView
    },
    {
        path: '/test',
        name: 'test',
        component: TestView
    },
    {
        path: '/settings',
        name: 'settings',
        component: SettingsView
    }
]

const router = createRouter({
    history: createWebHistory(), // In Tauri funktioniert WebHistory einwandfrei
    routes
})

export default router