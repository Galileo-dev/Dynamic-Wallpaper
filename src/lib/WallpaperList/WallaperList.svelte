<script>
	import Button from '$lib/button.svelte';
	import AddNewCard from './AddNewCard.svelte';

	import WallpaperCard from './WallpaperCard.svelte';
	// let template_item = { id: '12312abc', imagePath: '../pwpoe/woapk.png' };
	export let items = [];
	console.log(items);
	// for (let i = 0; i < 20; i++) {
	// 	items.push(template_item);
	// }
</script>

<div class="container">
	<div class="item-list">
		{#each items as item}
			<WallpaperCard
				image={'https://source.unsplash.com/random/?sexy'}
				width="300px"
				height="200px"
				name={item.id}
				{item}
			/>
		{/each}
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

	.item-list {
		display: grid;
		align-self: center;
		justify-items: center;
		grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));
		grid-row-gap: 1rem;
		grid-column-gap: 2rem;
		overflow-x: hidden;
		overflow-y: scroll;
		height: 100%;
		/* display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		grid-auto-rows: 50px;
		grid-gap: 10px;
		overflow-y: scroll; /*added*/
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
