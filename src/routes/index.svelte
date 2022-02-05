<script context="module" lang="ts">
	export const prerender = true;
	import CustomButton from '../lib/button.svelte';
	import WallpaperList from '../lib/WallpaperList/WallaperList.svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	function setBackground(imagePath) {
		invoke('set_wallpaper', { path: imagePath });
	}
</script>

<div class="container">
	<div class="scroll-div">
		<WallpaperList />
	</div>
	<div>
		<CustomButton text="Add" on:click={() => setBackground('hello')} />
	</div>
</div>

<style>
	.container {
		display: grid;
		height: 100%;
	}

	.scroll-div {
		height: 70vh;
		border: 3px solid red;
	}
</style>
