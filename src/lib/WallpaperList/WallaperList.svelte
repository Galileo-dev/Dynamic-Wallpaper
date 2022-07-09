<script>
	import Button from '$lib/button.svelte';
	import AddNewCard from './AddNewCard.svelte';
	import { loadData } from './api.svelte';

	import WallpaperCard from './WallpaperCard.svelte';
	let template_item = { id: '12312abc', imagePath: '../pwpoe/woapk.png' };
	export let items = [];
	// console.log(items);
	// for (let i = 0; i < 20; i++) {
	// 	items.push(template_item);
	// }
	// let url = 'https://source.unsplash.com/random/?=wallpaper';
</script>

<div class="container">
	<div class="item-list">
		<div class="items-container">
			{#each items as item}
				<WallpaperCard image={item.thumbnail} width="300px" height="200px" name={item.id} {item} />
			{/each}
		</div>
	</div>
</div>

<style>
	div::-webkit-scrollbar {
		width: 0.35rem;
	}

	div::-webkit-scrollbar-track {
		background-color: var(--tertiary-color);
		border-radius: 90px;
	}

	div::-webkit-scrollbar-thumb {
		background-color: var(--accent-color);
		border-radius: 90px;
	}

	.items-container {
		display: grid;
		align-self: center;
		justify-items: center;
		grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
		grid-row-gap: 1rem;
		grid-column-gap: 1rem;
		margin-top: 20px;
		margin-bottom: 50px;
	}

	.item-list {
		/* display: grid; */
		/* align-self: center;
		justify-items: center; */
		/* grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
		grid-row-gap: 1rem;
		grid-column-gap: 2rem; */
		overflow-x: hidden;
		overflow-y: scroll;
		height: 100%;
	}

	.container {
		height: 75%;
		/* border: 3px solid red; */
		scroll-behavior: smooth;
		margin-bottom: 50px;
		background-color: var(--tertiary-color);
		border-radius: 5px;
	}
</style>
