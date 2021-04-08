import { defineConfig } from "vite";
import svelte from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default ({ command, mode }) => {
  return { plugins: [svelte()], base: mode == "production" ? "/admin" : "/" };
};
