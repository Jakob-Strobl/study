import { defineConfig } from "vite";
import viteReact from "@vitejs/plugin-react";

export default defineConfig({
  server: {
    // Proxy all request to /api and /public on our dev server :5173 to our API server running on port 3000
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
      "/public": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
    },
  },
  plugins: [viteReact()],
});
