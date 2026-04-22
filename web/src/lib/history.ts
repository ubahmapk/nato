import { writable } from 'svelte/store';

export interface HistoryEntry {
	input: string;
	timestamp: number;
}

function createHistory() {
	const { subscribe, update, set } = writable<HistoryEntry[]>([]);
	return {
		subscribe,
		add(input: string) {
			update(entries => {
				// Skip consecutive duplicates
				if (entries.length > 0 && entries[0].input === input) return entries;
				return [{ input, timestamp: Date.now() }, ...entries];
			});
		},
		clear() {
			set([]);
		}
	};
}

export const history = createHistory();
