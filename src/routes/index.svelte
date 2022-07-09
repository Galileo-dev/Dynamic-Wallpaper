<script context="module" lang="ts">
	import CustomButton from '../lib/button.svelte';
	import WallpaperList from '../lib/WallpaperList/WallaperList.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { deleteData, addData, loadData } from '$lib/WallpaperList/api.svelte';

	async function getItems() {
		loadData();
		return;
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
		<CustomButton text="Hide Window" on:click={() => appWindow.hide()} />
	</div>
</div>

<style>
	.container {
		height: 100%;
		width: 100%;
		margin: 0;
		padding: 0;
		overflow: hidden;
	}
</style>
