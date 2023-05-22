<script lang="ts">
	import { Button, Group } from '@svelteuidev/core';
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import SavePathDisplay from './SavePathDisplay.svelte';

	let files: Awaited<ReturnType<typeof open>>;
	let folder: Awaited<ReturnType<typeof open>>;

	async function open_files() {
		files = await open({ multiple: true });
	}

	async function open_folder() {
		folder = await open({ directory: true });
	}

	async function e() {
		console.log('files', files);
		console.log('folder', folder);
		let a = await invoke('encrypt_files', { files, folder });
		console.log('e', a);
	}

	async function d() {
		console.log('files', files);
		console.log('folder', folder);
		let a = await invoke('decrypt_files', { files, folder });
		console.log('d', a);
	}
</script>

<div class="container">
	<div class="row row-1"><h1>Welcome to SvelteKit</h1></div>
	<div class="row row-2">
		<Group>
			<Button
				on:click={() => {
					open_files();
				}}
				variant="gradient"
				gradient={{ from: 'teal', to: 'green', deg: 105 }}
			>
				打开文件
			</Button>
			<Button
				on:click={() => {
					e();
				}}
				variant="gradient"
				gradient={{ from: 'orange', to: 'red', deg: 45 }}
			>
				E
			</Button>
			<Button
				on:click={() => {
					d();
				}}
				variant="gradient"
				gradient={{ from: 'grape', to: 'pink', deg: 35 }}
			>
				D
			</Button>
		</Group>
	</div>
	<div class="row row-3">
		<SavePathDisplay />
		<Button
			on:click={() => {
				open_folder();
			}}
			variant="gradient"
			gradient={{ from: 'teal', to: 'blue', deg: 60 }}
		>
			设置保存文件路径
		</Button>
	</div>
</div>

<style>
	.container {
		margin-top: 50px;
	}
	.row {
		margin-top: 20px;
	}
	h1 {
		color: #ffffff;
	}
</style>
