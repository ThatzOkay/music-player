import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const layoutComponents = {
    topBar: () => import('../components/TopBar.vue'),
    sidebar: () => import('../components/Sidebar.vue'),
    player: () => import('../components/Player.vue')
}

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        component: () => import('../views/FirstRunRouter.vue'),
        meta: {
            fullCenter: true
        }
    },
    {
        path: '/firstRun',
        component: () => import('../views/FirstRun.vue'),
        meta: {
            fullCenter: true
        }
    },
    {
        path: '/addSubsonic',
        component: () => import('../views/AddSubsonic.vue'),
        meta: {
            fullCenter: true
        }
    },
    {
        path: '/album/all',
        components: {
            default: () => import('../views/album/All.vue'),
            ...layoutComponents
        },
        meta: {
            breadcrumb: [
                "Albums",
                "All"
            ]
        }
    },
    {
        path: '/album/recentlyAdded',
        components: {
            default: () => import('../views/album/RecentlyAdded.vue'),
            ...layoutComponents
        },
        meta: {
            breadcrumb: [
                "Albums",
                "Recently Added"
            ]
        }
    }
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes
});

export default router;