import { svelte } from '@sveltejs/vite-plugin-svelte';
import rust from '@wasm-tool/rollup-plugin-rust';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';

export default ({ mode }) => {
	const isProduction = mode === 'production';

	return {
		root: 'src',
		plugins: [
			svelte({
				emitCss: true,
				preprocess: sveltePreprocess(),
				compilerOptions: {
					dev: !isProduction,
				},
			}),
			typescript(),
			rust({
				debug: false,
				watchPatterns: ['../**/*.rs'],
			}),
		],
		envDir: '..',
		publicDir: '../public',
		build: {
			outDir: '../dist',
			emptyOutDir: true,
			minify: isProduction,
		},
	};
};
