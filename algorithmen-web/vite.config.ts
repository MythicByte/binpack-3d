import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "npm:@tailwindcss/vite";
import wasm from "npm:vite-plugin-wasm";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss(), wasm()],
});
