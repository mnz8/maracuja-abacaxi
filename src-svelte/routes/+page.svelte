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
		let a = await invoke('encrypt_files', { files, folder });
	}

	async function d() {
		let a = await invoke('decrypt_files', { files, folder });
	}

	$: folder_path = (folder as string) || '默认为文件所在路径';
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
				选择文件
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
	<SavePathDisplay text={folder_path} />
	<div class="row row-3">
		<Group>
			<Button
				on:click={() => {
					open_folder();
				}}
				variant="gradient"
				gradient={{ from: 'teal', to: 'blue', deg: 60 }}
			>
				设置保存路径
			</Button>
			<Button
				on:click={() => {
					folder = null;
				}}
				variant="gradient"
				gradient={{ from: 'violet', to: 'indigo', deg: 60 }}
			>
				恢复默认路径
			</Button>
		</Group>
	</div>
</div>

<style>
	.container {
		margin-top: 50px;
		padding: 0 20px;
	}
	.row {
		margin-top: 20px;
	}
	h1 {
		color: #ffffff;
	}
</style>
