<script lang="ts">
	import Titlebar from '$lib/titlebar/titlebar.svelte';
	import '../app.css';
</script>

<Titlebar />

<svelte:body />
<div id="main">
	<slot />
</div>

<style>
	#main {
		height: calc(100% - 32px);
		margin-top: 32px;
		padding: 20px;
		overflow-y: auto;
	}

	:global(body) {
		border: 1px solid #48545c;
		overflow-y: hidden;
	}
	/* 
	footer {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 40px;
	}

	footer a {
		font-weight: bold;
	} */

	@media (min-width: 480px) {
		footer {
			padding: 40px 0;
		}
	}
</style>
