<script lang="ts">
	import { ensureInit, convert, type NatoEntry } from '$lib/nato.js';

	let inputText = $state('');
	let initialized = $state(false);

	$effect(() => {
		ensureInit().then(() => {
			initialized = true;
		});
	});

	let entries: NatoEntry[] = $derived(
		initialized && inputText.trim() ? convert(inputText) : []
	);
</script>

<main>
	<h1>NATO Phonetic Alphabet</h1>

	<label for="input">Enter text</label>
	<input
		id="input"
		type="text"
		bind:value={inputText}
		placeholder={initialized ? 'Type something…' : 'Loading…'}
		disabled={!initialized}
	/>

	{#if entries.length > 0}
		<ul>
			{#each entries as entry, i (i)}
				<li>
					<span class="char">{entry.character.toUpperCase()}</span>
					<span class="sep">–</span>
					<span class="word">{entry.word ?? '(no equivalent)'}</span>
				</li>
			{/each}
		</ul>
	{/if}
</main>

<style>
	main {
		max-width: 600px;
		margin: 2rem auto;
		padding: 0 1rem;
		font-family: system-ui, sans-serif;
	}

	label {
		display: block;
		font-weight: 600;
		margin-bottom: 0.25rem;
	}

	input {
		width: 100%;
		padding: 0.5rem 0.75rem;
		font-size: 1rem;
		border: 1px solid #ccc;
		border-radius: 4px;
		box-sizing: border-box;
	}

	input:disabled {
		background: #f5f5f5;
		color: #999;
	}

	ul {
		list-style: none;
		padding: 0;
		margin-top: 1.25rem;
	}

	li {
		display: flex;
		gap: 0.5rem;
		padding: 0.3rem 0;
		font-size: 1.05rem;
		border-bottom: 1px solid #f0f0f0;
	}

	.char {
		font-weight: 700;
		min-width: 2.5rem;
		font-family: monospace;
		font-size: 1.1rem;
	}

	.sep {
		color: #bbb;
	}

	.word {
		color: #222;
	}
</style>
