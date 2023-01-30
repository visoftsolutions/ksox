import { defineConfig } from "vite";
import solid from "solid-start/vite";
import { VitePWA } from "vite-plugin-pwa";
import type { VitePWAOptions } from "vite-plugin-pwa";

const pwaOptions: Partial<VitePWAOptions> = {
  registerType: "autoUpdate",
  workbox: {
    globPatterns: ["**/*.{js,css,html,ico,png,svg}"],
    additionalManifestEntries: [
      { revision: null, url: "/" },
      { revision: null, url: "index.html" },
    ],
  },
  manifest: {
    theme_color: "#0F0D12",
    name: "KSOX",
    short_name: "KSOX",
    description: "KSOX - Cryptocurrency Exchange & Payment Processor",
    icons: [
      {
        src: "/pwa/android-chrome-192x192.png",
        sizes: "192x192",
        type: "image/png",
        purpose: "maskable",
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
});
