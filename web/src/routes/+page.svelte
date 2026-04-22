<script lang="ts">
	import { ensureInit, convert, type NatoEntry } from '$lib/nato.js';
	import { formatEntries, type OutputFormat } from '$lib/format.js';
	import { history } from '$lib/history.js';

	const formats: OutputFormat[] = ['human', 'compact', 'json'];
	const formatLabels: Record<OutputFormat, string> = {
		human: 'Human',
		compact: 'Compact',
		json: 'JSON'
	};

	let inputText = $state('');
	let initialized = $state(false);
	let format = $state<OutputFormat>('human');
	let copied = $state(false);

	$effect(() => {
		ensureInit().then(() => {
			initialized = true;
		});
	});

	let entries: NatoEntry[] = $derived(
		initialized && inputText.trim() ? convert(inputText) : []
	);

	let formattedOutput = $derived(formatEntries(entries, format));

	// Add to history on Enter — avoids noisy per-keystroke entries.
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && inputText.trim()) {
			history.add(inputText.trim());
		}
	}

	async function copyToClipboard() {
		if (!formattedOutput) return;
		try {
			await navigator.clipboard.writeText(formattedOutput);
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 1500);
		} catch {
			// Clipboard API unavailable in insecure contexts
		}
	}
</script>

<main>
	<h1>NATO Phonetic Alphabet</h1>

	<label for="input">Enter text</label>
	<input
		id="input"
		type="text"
		bind:value={inputText}
		onkeydown={handleKeydown}
		placeholder={initialized ? 'Type something… (Enter to save to history)' : 'Loading…'}
		disabled={!initialized}
		autocomplete="off"
		spellcheck="false"
	/>

	{#if entries.length > 0}
		<fieldset class="format-toggle">
			<legend>Format</legend>
			<div class="toggle-options">
				{#each formats as f (f)}
					<label class="toggle-option" class:active={format === f}>
						<input type="radio" name="format" value={f} bind:group={format} />
						{formatLabels[f]}
					</label>
				{/each}
			</div>
		</fieldset>

		<div class="output-block">
			<pre class="output-text">{formattedOutput}</pre>
			<button class="copy-btn" class:copied onclick={copyToClipboard}>
				{copied ? '✓ Copied' : 'Copy'}
			</button>
		</div>
	{/if}

	{#if $history.length > 0}
		<details class="history-panel">
			<summary>History ({$history.length})</summary>
			<ul>
				{#each $history as entry, i (i)}
					<li>
						<button class="history-item" onclick={() => { inputText = entry.input; }}>
							{entry.input}
						</button>
					</li>
				{/each}
			</ul>
			<button class="clear-btn" onclick={() => history.clear()}>
				Clear history
			</button>
		</details>
	{/if}
</main>

<style>
	main {
		max-width: 600px;
		margin: 2rem auto;
		padding: 0 1rem;
		font-family: system-ui, sans-serif;
	}

	h1 {
		font-size: 1.5rem;
		margin-bottom: 1.25rem;
	}

	label {
		display: block;
		font-weight: 600;
		margin-bottom: 0.25rem;
	}

	input[type='text'] {
		width: 100%;
		padding: 0.5rem 0.75rem;
		font-size: 1rem;
		border: 1px solid #ccc;
		border-radius: 6px;
		box-sizing: border-box;
	}

	input[type='text']:disabled {
		background: #f5f5f5;
		color: #999;
	}

	/* Format toggle */
	.format-toggle {
		border: none;
		padding: 0;
		margin: 1.25rem 0 0.75rem;
	}

	.format-toggle legend {
		font-size: 0.8rem;
		font-weight: 600;
		color: #666;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.4rem;
	}

	.toggle-options {
		display: flex;
	}

	.toggle-option {
		cursor: pointer;
		font-weight: normal;
		margin-bottom: 0;
	}

	.toggle-option input[type='radio'] {
		position: absolute;
		opacity: 0;
		width: 0;
		height: 0;
	}

	/* Style label text as a segmented-control button */
	.toggle-option {
		display: inline-block;
		padding: 0.3rem 0.85rem;
		border: 1px solid #ccc;
		border-right: none;
		background: #fff;
		font-size: 0.9rem;
		color: #444;
		transition: background 0.1s, color 0.1s;
		user-select: none;
	}

	.toggle-option:first-child {
		border-radius: 6px 0 0 6px;
	}

	.toggle-option:last-child {
		border-right: 1px solid #ccc;
		border-radius: 0 6px 6px 0;
	}

	.toggle-option.active {
		background: #1a56db;
		border-color: #1a56db;
		color: #fff;
	}

	/* Output */
	.output-block {
		position: relative;
		margin-top: 0.5rem;
	}

	.output-text {
		background: #f8f8f8;
		border: 1px solid #e5e5e5;
		border-radius: 6px;
		padding: 0.75rem 1rem;
		margin: 0;
		font-family: ui-monospace, 'Cascadia Code', monospace;
		font-size: 0.95rem;
		line-height: 1.7;
		white-space: pre-wrap;
		word-break: break-word;
		min-height: 3rem;
	}

	.copy-btn {
		position: absolute;
		top: 0.5rem;
		right: 0.5rem;
		padding: 0.2rem 0.65rem;
		font-size: 0.8rem;
		border: 1px solid #ccc;
		border-radius: 4px;
		background: #fff;
		cursor: pointer;
		transition: background 0.15s;
	}

	.copy-btn:hover {
		background: #f0f0f0;
	}

	.copy-btn.copied {
		background: #d1fae5;
		border-color: #6ee7b7;
		color: #065f46;
	}

	/* History */
	.history-panel {
		margin-top: 2rem;
		border-top: 1px solid #e5e5e5;
		padding-top: 1rem;
	}

	.history-panel summary {
		cursor: pointer;
		font-size: 0.9rem;
		font-weight: 600;
		color: #555;
		user-select: none;
	}

	.history-panel ul {
		list-style: none;
		padding: 0;
		margin: 0.75rem 0 0;
	}

	.history-item {
		display: block;
		width: 100%;
		text-align: left;
		background: none;
		border: none;
		padding: 0.3rem 0.25rem;
		font-size: 0.9rem;
		color: #1a56db;
		cursor: pointer;
		border-radius: 3px;
	}

	.history-item:hover {
		background: #eff6ff;
	}

	.clear-btn {
		margin-top: 0.75rem;
		padding: 0.25rem 0.75rem;
		font-size: 0.8rem;
		border: 1px solid #fca5a5;
		border-radius: 4px;
		background: #fff;
		color: #dc2626;
		cursor: pointer;
	}

	.clear-btn:hover {
		background: #fef2f2;
	}
</style>
