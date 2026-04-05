import fs from 'fs-extra';
import { resolve, join } from 'path';

const root = resolve('.');
const rustDocSrc = join(root, 'target', 'doc');
const rustDocDest = join(root, 'docs', 'public', 'api', 'rust');

async function collectDocs() {
  try {
    console.log('Collecting Rust documentation...');
    if (await fs.pathExists(rustDocSrc)) {
      await fs.ensureDir(rustDocDest);
      await fs.copy(rustDocSrc, rustDocDest);
      console.log('Rust documentation copied to docs/public/api/rust/');
    } else {
      console.warn('Rust documentation not found in target/doc/.');
    }
  } catch (err) {
    console.error('Error collecting documentation:', err);
    process.exit(1);
  }
}

collectDocs();
