import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { init } from './vited.config.ts';

// https://vite.dev/config/
export default defineConfig(async () => {
    return {
        plugins: [vue()],
        server: {
            proxy: {
                '/api': {
                    target: 'http://localhost:3000',
                    changeOrigin: true,
                    rewrite: (path: string) => path.replace(/^\/api/, ''),
                    xfwd: true,
                },
            },
        },
    };
});
