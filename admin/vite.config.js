import svelte from "@sveltejs/vite-plugin-svelte";

// https://vitejs.dev/config/
export default ({ mode }) => {
  return { plugins: [svelte()], base: mode == "production" ? "/admin" : "/" };
};
