import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [

	],

	kit: {
		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html'
		}),

		vite: () => {
			plugins: [
				{
					name: 'watch-content',
					configureServer(server) {
						server.watcher.add(path.join(dirname, 'lib'));
					}
					// handleHotUpdate(ctx) {
					// 	let m = /(notes|posts|roam-pages)\/(.*)\.(md|html)$/.exec(ctx.file);
					// 	if (m) {
					// 		let contentType = m[1];
					// 		let id = m[2];

					// 		// This is just a conversion from the directory
					// 		// names to the URLs used in the site.
					// 		if (contentType === 'roam-pages') {
					// 			contentType = 'notes';
					// 		} else if (contentType === 'posts') {
					// 			contentType = 'writing';
					// 		}

					// 		ctx.server.ws.send({
					// 			type: 'custom',
					// 			event: 'content-update',
					// 			data: {
					// 				type: contentType,
					// 				id,
					// 			},
					// 		});

					// 		// Return an empty module list since we
					// 		// handled it manually.
					// 		return [];
					// 	}

					// 	// Not an event we care about, so just do
					// 	// the default behavior.
					// 	return ctx.modules;
					// },
				},
			];
		},
	}
}

export default config;
