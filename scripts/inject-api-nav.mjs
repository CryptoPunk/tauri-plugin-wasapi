import fs from 'fs';
import path from 'path';

const navFile = 'docs/api-nav.html';

async function injectNav() {
  if (!fs.existsSync(navFile)) {
    console.error(`Navigation file not found: ${navFile}`);
    process.exit(1);
  }

  const navHtml = fs.readFileSync(navFile, 'utf8');

  async function processDir(dir, type) {
    if (!fs.existsSync(dir)) return;

    const files = fs.readdirSync(dir);
    for (const file of files) {
      const filePath = path.join(dir, file);
      const stat = fs.statSync(filePath);

      if (stat.isDirectory()) {
        await processDir(filePath, type);
      } else if (file.endsWith('.html')) {
        let content = fs.readFileSync(filePath, 'utf8');
        
        // Remove any old injections
        content = content.replace(/<a href="\/" class="back-to-home">[\s\S]*?<\/a>/g, '');
        content = content.replace(/<style>\s*\.back-to-home[\s\S]*?<\/style>/g, '');

        if (type === 'rust') {
          // Rustdoc: Inject into sidebar
          if (content.includes('<nav class="sidebar">')) {
            content = content.replace('<nav class="sidebar">', `<nav class="sidebar">${navHtml}`);
          }
        } else if (type === 'ts') {
          // TypeDoc: Inject into the search bar container as the first element
          if (content.includes('id="tsd-search"')) {
            content = content.replace(/(<div[^>]*id="tsd-search"[^>]*>)/, `$1${navHtml}`);
          }
        }

        fs.writeFileSync(filePath, content, 'utf8');
      }
    }
  }

  console.log('Injecting navigation into TypeScript documentation...');
  await processDir('docs/public/api/ts', 'ts');
  
  console.log('Injecting navigation into Rust documentation...');
  await processDir('docs/public/api/rust/doc', 'rust');
  
  console.log('Done.');
}

injectNav();
