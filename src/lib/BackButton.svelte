<script lang="ts">
	export let text = 'Create';
	import Icon from '@iconify/svelte';
	import { goto } from '$app/navigation';

	function goBack(defaultRoute = '/') {
		const ref = document.referrer;
		goto(ref.length > 0 ? ref : defaultRoute);
	}
</script>

<div class="container" on:click={() => goBack()}>
	<Icon icon="ion:caret-back-circle" width="2.5rem" />
</div>

<style>
	.disable-select {
		user-select: none; /* supported by Chrome and Opera */
		-webkit-user-select: none; /* Safari */
		-khtml-user-select: none; /* Konqueror HTML */
		-moz-user-select: none; /* Firefox */
		-ms-user-select: none; /* Internet Explorer/Edge */
	}

	.container {
		display: flex;
		align-self: center;
		color: var(--accent-color);
	}
</style>
