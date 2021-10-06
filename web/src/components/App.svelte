<section class="convert-office-images-app">
	<form on:submit={handleSubmit} class:loading>
		<label>
			<span>
				Drop file
				<small>
					or click to select
				</small>
			</span>

			<input type="file" name="file" accept=".docx,.pptx" required />
		</label>

		<button type="submit" disabled={loading}>Submit</button>
	</form>

	<p>
		Select a large Word (.docx) or PowerPoint (.pptx) document, press
		submit, and receive a new, smaller document more convenient for
		sharing.
	</p>

	{#if error}
		{#if error.message === 'No file'}
			<p>
				Please select a Word or PowerPoint file to convert its images.
			</p>
		{:else}
			<p>Sorry, there was a problem with that file.</p>
		{/if}
	{/if}

</section>

<script>
	import download from 'downloadjs';

	import init from '../wasm-wrapper.js';

	let loading = false;
	let error = null;

	async function handleSubmit(event) {
		event.preventDefault();

		loading = true;

		try {
			const form = event.target;
			const file = form.elements['file'].files[0];

			if (!file) {
				throw new Error('No file');
			}

			const [{ convert_images }, arrayBuffer] = await Promise.all([init(), file.arrayBuffer()]);

			const output = convert_images(new Uint8Array(arrayBuffer));
			download(output, file.name);
		} catch (e) {
			console.error(e);
			error = e;
		}

		loading = false;
	}
</script>

<style>
	form {
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
		margin-bottom: 3em;
	}

	form.loading {
		opacity: 0.75;
	}

	form label {
		font-size: 2em;
		box-sizing: border-box;
		display: flex;
		flex-direction: column;
		align-items: space-between;
		justify-content: space-between;
		padding: 3em;
		width: 60%;
		height: 500px;
		max-height: 80vh;
		margin: 1em;
		background-color: #185abd;
		border-radius: 1px;
		cursor: pointer;
		color: white;
	}

	label:hover {
		background: #2b7cd3;
	}

	form.loading label {
		cursor: wait;
	}

	small {
		opacity: 0.5;
	}
</style>
