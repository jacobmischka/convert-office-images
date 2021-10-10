import { svelte } from '@sveltejs/vite-plugin-svelte';
import rust from '@wasm-tool/rollup-plugin-rust';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';
import { VitePWA } from 'vite-plugin-pwa';

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
			VitePWA({
				registerType: 'autoUpdate',
				includeAssets: [
					'selawk.ttf',
					'icon.svg',
					'favicon.ico',
					'apple-touch-icon.png',
					'robots.txt',
				],
				manifest: {
					name: 'Convert Office Images',
					short_name: 'Convert Office Images',
					description:
						'Converts all images in a Microsoft Word or PowerPoint document to JPEGs to reduce document file size',
					theme_color: '#185abd',
					icons: [
						{
							src: 'icon-192.png',
							sizes: '192x192',
							type: 'image/png',
						},
						{
							src: 'icon-512.png',
							sizes: '512x512',
							type: 'image/png',
						},
						{
							src: 'icon-512.png',
							sizes: '512x512',
							type: 'image/png',
							purpose: 'any maskable',
						},
					],
				},
				workbox: {
					globPatterns: [
						'**/*.js',
						'**/*.css',
						'**/*.html',
						'**/*.wasm',
						'**/*.svg',
						'**/*.ico',
						'**/*.png',
					],
				},
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
