import { defineConfig } from 'vite';
import { copyFileSync, cpSync } from 'fs';
import { resolve } from 'path';

export default defineConfig({
  root: './dist',
  build: {
    outDir: '../build',           // 输出到项目根目录下的 build 文件夹（避免覆盖原始 dist）
    emptyOutDir: true,
    rollupOptions: {
      input: './dist/index.html'
    }
  },
  server: {
    port: 5173,
    strictPort: true,
  },
  plugins: [
    {
      name: 'copy-static',
      closeBundle() {
        // 复制 l2d.js 和 cfitchan-0504 到输出目录
        const srcDir = resolve(__dirname, 'dist');
        const outDir = resolve(__dirname, 'build');
        cpSync(resolve(srcDir, 'l2d.js'), resolve(outDir, 'l2d.js'), { force: true });
        cpSync(resolve(srcDir, 'cfitchan-0504'), resolve(outDir, 'cfitchan-0504'), { recursive: true, force: true });
        console.log('Copied l2d.js and cfitchan-0504 to build/');
      }
    }
  ]
});