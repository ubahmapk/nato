import type { NatoEntry } from './nato.js';

export type OutputFormat = 'human' | 'compact' | 'json';

export function formatEntries(entries: NatoEntry[], format: OutputFormat): string {
	if (entries.length === 0) return '';

	switch (format) {
		case 'human':
			return entries
				.map(e => `${e.character.toUpperCase()} – ${e.word ?? '(no equivalent)'}`)
				.join('\n');
		case 'compact':
			return entries
				.filter(e => e.word !== null)
				.map(e => e.word as string)
				.join(' ');
		case 'json':
			return JSON.stringify(
				entries.map(e => ({ character: e.character, word: e.word })),
				null,
				2
			);
	}
}
