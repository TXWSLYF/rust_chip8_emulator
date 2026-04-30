import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { fileURLToPath, URL } from 'node:url';

export default defineConfig(() => {
  const isGitHubActions = process.env.GITHUB_ACTIONS === 'true';
  const repoName = process.env.GITHUB_REPOSITORY?.split('/')[1];

  return {
    // GitHub Pages uses /<repo>/ as base path
    base: isGitHubActions && repoName ? `/${repoName}/` : '/',
    root: fileURLToPath(new URL('./web', import.meta.url)),
    publicDir: fileURLToPath(new URL('./roms', import.meta.url)),
    plugins: [react(), tailwindcss()],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./web/src', import.meta.url)),
      },
    },
    server: {
      host: '0.0.0.0',
      port: 5173,
    },
    preview: {
      host: '0.0.0.0',
      port: 4173,
    },
    build: {
      outDir: 'dist',
      emptyOutDir: true,
    },
  };
});
