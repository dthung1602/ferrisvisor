import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
        configure: (proxy) => {
          proxy.on("proxyRes", (proxyRes) => {
            let body = Buffer.from([]);

            // 1. Collect the data chunks
            proxyRes.on("data", (chunk) => {
              body = Buffer.concat([body, chunk]);
            });

            // 2. When the stream ends, process it
            proxyRes.on("end", () => {
              const responseString = body.toString("utf8");
              console.log("Response Body:", responseString);
            });
          });
        }
      }
    }
  }
});
