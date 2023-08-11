import { defineConfig } from "vite";
import solid from "solid-start/vite";
import { VitePWA } from "vite-plugin-pwa";
import type { VitePWAOptions } from "vite-plugin-pwa";
import { createHash, randomBytes } from "crypto";

const buildHash = createHash("md5").update(randomBytes(20)).digest("hex");

const pwaOptions: Partial<VitePWAOptions> = {
  registerType: "autoUpdate",
  workbox: {
    globPatterns: ["**/*.{js,css,html,ico,png,svg,ttf}"],
    navigateFallback: null,
    cleanupOutdatedCaches: true,
    additionalManifestEntries: [{ revision: buildHash, url: "/" }],
  },
  manifest: {
    theme_color: "#0F0D12",
    name: "KSOX",
    short_name: "KSOX",
    description: "KSOX - Cryptocurrency Exchange & Payment Processor",
    icons: [
      {
        src: "/pwa/mstile-150x150-darkbg.png",
        sizes: "150x150",
        type: "image/png",
        purpose: "maskable",
      },
      {
        src: "/pwa/android-chrome-192x192.png",
        sizes: "192x192",
        type: "image/png",
      },
      {
        src: "/pwa/android-chrome-512x512.png",
        sizes: "512x512",
        type: "image/png",
      },
    ],
  },
};

export default defineConfig({
  plugins: [solid(), VitePWA(pwaOptions)],
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080/',
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    }
  }
});
