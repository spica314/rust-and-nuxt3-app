import wasmpack from 'vite-plugin-wasm-pack';

export default defineNuxtConfig({
  ssr: false,
  vite: {
    plugins: [wasmpack('../app-front-rust')]
  }
});
