import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import { init } from './vited.config.ts';
import { createSvgIconsPlugin } from 'vite-plugin-svg-icons';
import path from 'path';

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
