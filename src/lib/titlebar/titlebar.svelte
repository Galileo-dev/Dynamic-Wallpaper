<script>
	import { appWindow } from '@tauri-apps/api/window';
</script>

<div id="titlebar">
	<div id="drag-region" data-tauri-drag-region>
		<div id="window-title" data-tauri-drag-region>
			<span>Dynamic Wallpaper</span>
			<div id="right-title">
				<div id="version">
					<p>v0.0.0</p>
					<div id="divider" />
				</div>
			</div>
		</div>

		<div id="window-controls">
			<div class="button" id="min-button" on:click={() => appWindow.minimize()}>
				<img
					class="icon"
					srcset="/icons/min-w-10.png 1x, /icons/min-w-12.png 1.25x, /icons/min-w-15.png 1.5x, /icons/min-w-15.png 1.75x, /icons/min-w-20.png 2x, /icons/min-w-20.png 2.25x, /icons/min-w-24.png 2.5x, /icons/min-w-30.png 3x, /icons/min-w-30.png 3.5x"
					draggable="false"
				/>
			</div>
			<div class="button" id="max-button" on:click={() => appWindow.toggleMaximize()}>
				<img
					class="icon"
					srcset="/icons/max-w-10.png 1x, /icons/max-w-12.png 1.25x, /icons/max-w-15.png 1.5x, /icons/max-w-15.png 1.75x, /icons/max-w-20.png 2x, /icons/max-w-20.png 2.25x, /icons/max-w-24.png 2.5x, /icons/max-w-30.png 3x, /icons/max-w-30.png 3.5x"
					draggable="false"
				/>
			</div>
			<div class="button" id="close-button" on:click={() => appWindow.close()}>
				<img
					class="icon"
					srcset="/icons/close-w-10.png 1x, /icons/close-w-12.png 1.25x, /icons/close-w-15.png 1.5x, /icons/close-w-15.png 1.75x, /icons/close-w-20.png 2x, /icons/close-w-20.png 2.25x, /icons/close-w-24.png 2.5x, /icons/close-w-30.png 3x, /icons/close-w-30.png 3.5x"
					draggable="false"
				/>
			</div>
			<!-- <div class="button" id="restore-button">
				<img
					class="icon"
					srcset="/icons/restore-w-10.png 1x, /icons/restore-w-12.png 1.25x, /icons/restore-w-15.png 1.5x, /icons/restore-w-15.png 1.75x, /icons/restore-w-20.png 2x, /icons/restore-w-20.png 2.25x, /icons/restore-w-24.png 2.5x, /icons/restore-w-30.png 3x, /icons/restore-w-30.png 3.5x"
					draggable="false"
				/>
			</div> -->
		</div>
	</div>
</div>

<style>
	#right-title {
		align-items: center;
		display: flex;
		margin-left: auto;
		flex-wrap: nowrap;
		align-content: center;
		align-self: center;

		height: 100%;
	}

	#version {
		display: flex;
		flex-wrap: nowrap;
		align-content: center;
		align-self: center;

		height: 100%;
	}

	#version p {
		font-size: 1em;
		align-content: center;
		align-self: center;
		text-align: center;
		font-size: 1.2em;
		letter-spacing: 1.3px;
		font-weight: 400;
		margin: 1px;
	}

	#divider {
		border-left: 1px solid rgb(230, 230, 230);

		margin-left: 20px;
		margin-right: 20px;
		height: 15px;
		align-self: center;
	}

	#titlebar {
		user-select: none;
		display: flex;
		position: fixed;
		height: 32px;
		width: 100%;
	}

	.maximized #titlebar {
		width: 100%;
		padding: 0;
	}

	#main {
		height: calc(100% - 32px);
		margin-top: 32px;
		padding: 20px;
		overflow-y: auto;
	}

	/* #titlebar {
		padding: 4px;
	} */

	#titlebar #drag-region {
		width: 100%;
		height: 100%;
	}

	#titlebar {
		background: var(--main-hover-color);
		color: #fff;
	}

	#titlebar #drag-region {
		display: grid;
		grid-template-columns: auto 138px;
	}

	#window-title {
		grid-column: 1;
		display: flex;
		align-items: center;
		margin-left: 8px;
		overflow: hidden;
		font-family: 'Segoe UI', sans-serif;
		font-size: 12px;
	}

	.maximized #window-title {
		margin-left: 12px;
	}

	#window-title span {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
		line-height: 1.5;
	}

	#window-controls {
		display: grid;
		grid-template-columns: repeat(3, 46px);
		position: absolute;
		top: 0;
		right: 0;
		height: 100%;
	}

	#window-controls {
		-webkit-app-region: no-drag;
	}

	#window-controls .button {
		grid-row: 1 / span 1;
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
	}

	@media (-webkit-device-pixel-ratio: 1.5),
		(device-pixel-ratio: 1.5),
		(-webkit-device-pixel-ratio: 2),
		(device-pixel-ratio: 2),
		(-webkit-device-pixel-ratio: 3),
		(device-pixel-ratio: 3) {
		#window-controls .icon {
			width: 10px;
			height: 10px;
		}
	}

	#window-controls .button {
		user-select: none;
	}

	#window-controls .button:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	#window-controls .button:active {
		background: rgba(255, 255, 255, 0.2);
	}

	#close-button:hover {
		background: #e81123 !important;
	}

	#close-button:active {
		background: #f1707a !important;
	}
	#close-button:active .icon {
		filter: invert(1);
	}

	#min-button {
		grid-column: 1;
	}
	#max-button,
	#restore-button {
		grid-column: 2;
	}
	#close-button {
		grid-column: 3;
	}

	#restore-button {
		display: none !important;
	}

	.maximized #restore-button {
		display: flex !important;
	}

	.maximized #max-button {
		display: none;
	}
</style>
