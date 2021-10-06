import App from './components/App.svelte';

function createApp(target) {
	return new App({
		target
	});
}

export { App, createApp };

