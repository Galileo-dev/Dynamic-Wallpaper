<script context="module">
	// @ts-ignore
	import { Store } from 'tauri-plugin-store-api';
	const store = new Store('.settings');

	export async function loadData() {
		// Template
		// past_wallpaper: [
		// 		{
		// 			id: 'phhsfafuijo',
		// 			title: 'Sun Set',
		// 			date: '01/02/2004',
		// 			dir: 'C:/files/dir'
		// 		}
		// 	]
		let past_wallpaper = [];
		try {
			past_wallpaper = await store.get('past_wallpaper');
		} catch {
			console.error("could'nt load past wallpapers");
		}

		console.log(past_wallpaper);
		return past_wallpaper;
	}

	export async function addData() {
		let id = Math.floor(100000 + Math.random() * 900000);

		let template = {
			id,
			title: 'Sun Set',
			date: '01/02/2004',
			dir: 'C:/files/dir'
		};
		let previous = [];
		try {
			previous = await store.get('past_wallpaper');
		} catch {
			console.log('no preivous wallpapers');
		}

		let current = [];
		if (previous) {
			current = previous;
		} else {
			current = [];
		}
		current.push(template);
		await store.set('past_wallpaper', current).catch("Could'nt set store");
		store.save();
		console.log('Added New Wallpaper!');
	}

	export function deleteData() {
		return 1;
	}
</script>
