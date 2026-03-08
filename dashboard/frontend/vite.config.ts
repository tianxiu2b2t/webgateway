import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
// import { init } from './vited.config.ts';
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons';
import path from 'path';

// env
const backendUrl = process.env.BACKEND_URL || 'http://localhost:3000';

// https://vite.dev/config/
export default defineConfig(async () => {
    return {
        plugins: [
            createSvgIconsPlugin({
                iconDirs: [path.resolve(process.cwd(), 'src/assets/icons')],
                symbolId: 'icon-[dir]-[name]',
            }),
            vue(),
        ],
        server: {
            host: '0.0.0.0',
            proxy: {
                '/api': {
                    target: backendUrl,
                    changeOrigin: true,
                    rewrite: (path: string) => path.replace(/^\/api/, ''),
                    xfwd: true,
                },
            },
        },
    };
});
