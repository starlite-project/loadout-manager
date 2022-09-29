import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
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
		// minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
		minify: false,
		sourcemap: !!process.env.TAURI_DEBUG,
	},
	plugins: [react()]
});