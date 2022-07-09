<script context="module" type="ts">
	// @ts-ignore
	import { dialog } from '@tauri-apps/api';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Store } from 'tauri-plugin-store-api';
	const store = new Store('.settings.dat');

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
		let filter1: dialog.DialogFilter = {
			name: 'images',
			extensions: ['heic']
		};

		let dialogOptions: dialog.OpenDialogOptions = {
			filters: [filter1]
		};

		let path = await dialog.open(dialogOptions);

		console.log(path);

		let id = Math.floor(100000 + Math.random() * 900000);

		var today = new Date();
		var dd = String(today.getDate()).padStart(2, '0');
		var mm = String(today.getMonth() + 1).padStart(2, '0'); //January is 0!
		var yyyy = today.getFullYear();

		var date = mm + '/' + dd + '/' + yyyy;

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
		let id_string = id.toString();

		let json_data = await invoke('decode_heic', { pathToHeic: path, id: id_string });

		let template = {
			id,
			title: 'This will be set later',
			date: date,
			data: json_data
		};

		current.push(template);

		await store.set('past_wallpaper', current).catch("Could'nt set store");
		store.save();
		console.log('Added New Wallpaper!');
	}

	export function deleteData() {
		return 1;
	}
</script>
