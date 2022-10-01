import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';
import sassDts from 'vite-plugin-sass-dts';

// https://vitejs.dev/config/
export default defineConfig({
	css: {
		modules: {
			generateScopedName: process.env.TAURI_DEBUG ? '[name]__[local]__[hash:base64:5]' : '[hash:base64:5]'
		},
		preprocessorOptions: {
			scss: {
				importer(...args: string[]) {
					console.log(args);
					if (args[0] !== '@/styles') {
						return;
					}

					return {
						file: resolve(__dirname, 'src', 'assets', 'styles')
					}
				}
			}
		}
	},
	clearScreen: false,
	server: {
		strictPort: true,
	},
	resolve: {
		alias: {
			'react-redux': 'react-redux/dist/react-redux.js'
		}
	},
	define: {
		__LOADOUT_MANAGER_FLAVOR__: JSON.stringify(process.env.TAURI_DEBUG ? 'dev' : 'prod')
	},
	envPrefix: ['VITE_', 'TAURI_'],
	build: {
		target: ['es2021', 'chrome100', 'safari13'],
		minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		sourcemap: !!process.env.TAURI_DEBUG,
	},
	plugins: [
		react(),
		sassDts({
			enabledMode: ['development', 'production'],
		})
	]
});