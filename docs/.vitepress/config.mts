import { defineConfig } from 'vitepress'
import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid(
  defineConfig({
    title: "WASAPI Audio Capture",
    description: "Tauri 2 plugin for capturing audio via WASAPI",
    themeConfig: {
      nav: [
        { text: 'Home', link: '/' },
        { text: 'Guides', link: '/architecture' },
        { text: 'Rust API', link: '/api/rust/doc/tauri_plugin_wasapi/index.html', target: '_blank' },
        { text: 'TypeScript API', link: '/api/ts/index.html', target: '_blank' },
      ],
      sidebar: [
        {
          text: 'Introduction',
          items: [
            { text: 'What is this?', link: '/' },
            { text: 'Architecture', link: '/architecture' },
          ]
        },
        {
          text: 'User Guides',
          items: [
            { text: 'Permissions', link: '/permissions' },
            { text: 'Troubleshooting', link: '/troubleshooting' },
          ]
        },
        {
          text: 'Development',
          items: [
            { text: 'Contributing', link: '/contributing' },
          ]
        }
      ],
      socialLinks: [
        { icon: 'github', link: 'https://github.com/cryptopunk/tauri-plugin-wasapi' }
      ]
    },
    // Mermaid configuration
    mermaid: {
      // theme: 'forest',
    }
  })
)
