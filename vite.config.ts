import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import sassDts from 'vite-plugin-sass-dts';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
	clearScreen: false,
	server: {
		strictPort: true,
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
	plugins: [react(), sassDts({
		enabledMode: ['development', 'production'],
		global: {
			generate: true,
			outFile: path.resolve(__dirname, './src/style.d.ts'),
		}
	})]
});