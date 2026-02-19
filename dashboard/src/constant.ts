import ky from 'ky';
import type { APIResponse } from './types';
import { createRouter, createWebHistory } from 'vue-router';
import { checkToken } from './auth';

let inputId = 0;
export function increaseInputID() {
    return inputId++;
}

export const got = ky.create({
    prefixUrl: new URL('/api', window.location.origin).toString(),
    throwHttpErrors: false,
    hooks: {
        afterResponse: [
            async (_, __, response) => {
                let resp: APIResponse = {
                    status: response.status,
                    message: response.statusText,
                    data: null,
                };
                try {
                    resp = await response.json();
                } catch (e) {}
                return new Response(JSON.stringify(resp), response);
            },
        ],
    },
});

export const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/login',
            component: () => import('./pages/Auth.vue'),
        },
        {
            // 泛匹配 /xxx/xxx
            path: '/:pathMatch(.*)*',
            component: () => import('./pages/Dashboard.vue'),
            meta: {
                requiresAuth: true,
            },
            children: [
                {
                    path: 'settings',
                    component: () => import('./pages/Dashboard/Settings.vue'),
                },
                {
                    path: 'websites',
                    component: () => import('./pages/Dashboard/websites.vue'),
                },
                {
                    path: 'websites/certificates',
                    component: () =>
                        import('./pages/Dashboard/websites/Certificates.vue'),
                },
            ],
        },
    ],
});

router.beforeEach(async (to, _, next) => {
    if (to.meta.requiresAuth && !(await checkToken())) {
        next('/login');
    } else {
        next();
    }
});
