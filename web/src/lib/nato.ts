import init, { convert_to_nato } from 'nato-wasm';

export interface NatoEntry {
	character: string;
	word: string | null;
}

let initialized = false;

export async function ensureInit(): Promise<void> {
	if (!initialized) {
		await init();
		initialized = true;
	}
}

// DoS guard: cap input length before passing to WASM.
// The Rust layer is memory-safe regardless, but this prevents
// the browser from hanging on absurdly long inputs.
export function convert(input: string): NatoEntry[] {
	return convert_to_nato(input.slice(0, 10_000)) as NatoEntry[];
}
