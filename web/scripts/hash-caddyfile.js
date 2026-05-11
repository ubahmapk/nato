import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

const html = readFileSync('build/index.html', 'utf8');
const match = html.match(/<script>([\s\S]*?)<\/script>/);
if (!match) throw new Error('No inline script found in build/index.html');

const hash = createHash('sha256').update(match[1]).digest('base64');
const template = readFileSync('Caddyfile.template', 'utf8');
writeFileSync('Caddyfile', template.replace('INLINE_SCRIPT_HASH', `'sha256-${hash}'`));
