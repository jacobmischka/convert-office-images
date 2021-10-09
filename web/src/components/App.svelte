<section class="convert-office-images-app">
	<form on:submit={handleSubmit} class:loading>
		<label class="file-input">
			<span>
				Drop file
				<small>
					or click to select
				</small>
			</span>

			<input type="file" name="file" accept=".docx,.pptx" required />
		</label>

		<div class="row">
			<label>
				<input type="checkbox" name="reencode_jpegs" />
				Re-encode existing JPEGS
			</label>

			<label>
				Image quality
				<input type="number" name="quality" min="1" max="100" step="1" value="90" />
			</label>
		</div>

		<button type="submit" disabled={loading}>Submit</button>
	</form>

	{#if error}
		{#if error.message === 'No file'}
			<p>
				Please select a Word or PowerPoint file to convert its images.
			</p>
		{:else if error.message === 'Invalid quality'}
			<p>Quality must be a number between 1 and 100.</p>
		{:else}
			<p>Sorry, there was a problem with that file.</p>
		{/if}
	{/if}

	<p>
		Select a large Word (.docx) or PowerPoint (.pptx) document, press
		submit, and receive a new, smaller document more convenient for
		sharing.
	</p>

	<details>
		<summary>More information</summary>

		<p>Converts all images in your document to compressed JPEGS, resulting in a smaller file size.</p>

		<dl>
			<dt>Re-encode existing JPEGS</dt>
			<dd>
				If checked, images that are already JPEGS will be re-encoded.
				Use this if the document is still too large and you want to
				re-compress them at a lower quality.
			</dd>

			<dt>Image quality</dt>
			<dd>
				A number between 1 and 100.
				Lower quality results in a smaller file.
				100 is typically about double the size of 90 with little
				visual difference.
				You probably don't want to go lower than 90.
			</dd>
		</dl>
	</details>
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
			const file = form.elements.file.files[0];
			const quality = Number(form.elements.quality.value);
			const reencodeJpegs = form.elements.reencode_jpegs.checked;

			if (Number.isNaN(quality) || !Number.isInteger(quality) || quality < 1 || quality > 100) {
				throw new Error('Invalid quality');
			}

			if (!file) {
				throw new Error('No file');
			}

			const [{ convert_images }, arrayBuffer] = await Promise.all([init(), file.arrayBuffer()]);

			const output = convert_images(new Uint8Array(arrayBuffer), quality, reencodeJpegs);
			download(output, file.name);
		} catch (e) {
			console.error(e);
			error = e;
		}

		loading = false;
	}
</script>

<style>
	section {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	form {
		width: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-around;
		align-items: center;
		margin-bottom: 2em;
	}

	form.loading {
		opacity: 0.75;
	}

	form label.file-input {
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
		background-color: #185abd;
		border-radius: 1px;
		cursor: pointer;
		color: white;
	}

	form > * ~ * {
		margin-top: 1em;
	}

	label.file-input:hover {
		background: #2b7cd3;
	}

	form.loading {
		cursor: wait;
	}

	.row > label {
		margin: 0.5em 3em;
	}

	input[type="number"] {
		width: 4em;
	}

	small {
		opacity: 0.5;
	}

	details {
		margin: 1em;
		color: #333;
	}

	summary {
		cursor: pointer;
	}

	details p {
		margin: 1em 0;
	}

	dl {
		display: flex;
		flex-wrap: wrap;
		text-align: left;
		justify-content: center;
	}

	dt {
		font-weight: bold;
		flex-basis: 20%;
		margin: 0.5em;
	}

	dd {
		flex-basis: 60%;
		margin: 0.5em;
	}

	dt::after {
		content: ':'
	}
</style>
