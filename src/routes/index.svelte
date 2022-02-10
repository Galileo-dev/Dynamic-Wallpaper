<script context="module" lang="ts">
	import CustomButton from '../lib/button.svelte';
	import WallpaperList from '../lib/WallpaperList/WallaperList.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import { loadData, deleteData, addData } from '$lib/WallpaperList/api.svelte';

	async function getItems() {
		return loadData();
	}

	function setBackground(imagePath) {
		invoke('set_wallpaper', { path: imagePath });
	}

	let items = getItems();
</script>

<div class="container">
	{#await items then value}
		<WallpaperList items={value} />
	{/await}
	<div>
		<CustomButton text="Add" on:click={() => addData()} />
	</div>
</div>

<style>
	.container {
		display: grid;
		height: 100%;
	}
</style>
