import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import sassDts from 'vite-plugin-sass-dts';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
	css: {
		preprocessorOptions: {
			scss: {
				additionalData: `@use "@/styles" as common;`,
				importer(...args: string[]) {
					if (args[0] !== '@/styles') {
						return;
					}

					return {
						file: `${path.resolve(__dirname, './src/assets/styles')}`
					}
				}
			}
		}
	},
	clearScreen: false,
	server: {
		strictPort: true,
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		target: ['es2021', 'chrome100', 'safari13'],
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_DEBUG
	},
	plugins: [react(), sassDts({
		enabledMode: ['development', 'production'],
		global: {
			generate: true,
			outFile: path.resolve(__dirname, './src/style.d.ts'),
		}
	})]
});