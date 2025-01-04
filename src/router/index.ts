import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        component: () => import('../views/FirstRunRouter.vue')
    },
    {
        path: '/firstRun',
        component: () => import('../views/FirstRun.vue')
    },
    {
        path: '/addSubsonic',
        component: () => import('../views/AddSubsonic.vue')
    },
    {
        path: '/home',
        component: () => import('../views/Home.vue')
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
});

export default router;