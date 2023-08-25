// vite.config.ts
import { defineConfig } from "file:///home/bartosz/workshop/visoftsolutions/ksox/web/node_modules/vite/dist/node/index.js";
import { VitePWA } from "file:///home/bartosz/workshop/visoftsolutions/ksox/web/node_modules/vite-plugin-pwa/dist/index.js";
import solid from "file:///home/bartosz/workshop/visoftsolutions/ksox/web/node_modules/solid-start/vite/plugin.js";
import { randomBytes } from "crypto";
var buildHash = randomBytes(32).toString("hex");
var pwaOptions = {
  registerType: "autoUpdate",
  workbox: {
    globPatterns: ["**/*.{js,css,html,ico,png,svg,ttf}"],
    navigateFallback: null,
    cleanupOutdatedCaches: true,
    additionalManifestEntries: [{ revision: buildHash, url: "/" }]
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
        purpose: "maskable"
      },
      {
        src: "/pwa/android-chrome-192x192.png",
        sizes: "192x192",
        type: "image/png"
      },
      {
        src: "/pwa/android-chrome-512x512.png",
        sizes: "512x512",
        type: "image/png"
      }
    ]
  }
};
var vite_config_default = defineConfig({
  plugins: [solid(), VitePWA(pwaOptions)],
  envDir: "../../",
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:8080/",
        changeOrigin: true,
        rewrite: (path) => path.replace(/^\/api/, "")
      }
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9iYXJ0b3N6L3dvcmtzaG9wL3Zpc29mdHNvbHV0aW9ucy9rc294L3dlYi9hcHBzL3Byb2Nlc3NvclwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiL2hvbWUvYmFydG9zei93b3Jrc2hvcC92aXNvZnRzb2x1dGlvbnMva3NveC93ZWIvYXBwcy9wcm9jZXNzb3Ivdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL2hvbWUvYmFydG9zei93b3Jrc2hvcC92aXNvZnRzb2x1dGlvbnMva3NveC93ZWIvYXBwcy9wcm9jZXNzb3Ivdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tIFwidml0ZVwiO1xuaW1wb3J0IHsgVml0ZVBXQSwgdHlwZSBWaXRlUFdBT3B0aW9ucyB9IGZyb20gXCJ2aXRlLXBsdWdpbi1wd2FcIjtcbmltcG9ydCBzb2xpZCBmcm9tIFwic29saWQtc3RhcnQvdml0ZVwiO1xuaW1wb3J0IHsgcmFuZG9tQnl0ZXMgfSBmcm9tIFwiY3J5cHRvXCI7XG5cbmNvbnN0IGJ1aWxkSGFzaCA9IHJhbmRvbUJ5dGVzKDMyKS50b1N0cmluZyhcImhleFwiKTtcblxuY29uc3QgcHdhT3B0aW9uczogUGFydGlhbDxWaXRlUFdBT3B0aW9ucz4gPSB7XG4gIHJlZ2lzdGVyVHlwZTogXCJhdXRvVXBkYXRlXCIsXG4gIHdvcmtib3g6IHtcbiAgICBnbG9iUGF0dGVybnM6IFtcIioqLyoue2pzLGNzcyxodG1sLGljbyxwbmcsc3ZnLHR0Zn1cIl0sXG4gICAgbmF2aWdhdGVGYWxsYmFjazogbnVsbCxcbiAgICBjbGVhbnVwT3V0ZGF0ZWRDYWNoZXM6IHRydWUsXG4gICAgYWRkaXRpb25hbE1hbmlmZXN0RW50cmllczogW3sgcmV2aXNpb246IGJ1aWxkSGFzaCwgdXJsOiBcIi9cIiB9XSxcbiAgfSxcbiAgbWFuaWZlc3Q6IHtcbiAgICB0aGVtZV9jb2xvcjogXCIjMEYwRDEyXCIsXG4gICAgbmFtZTogXCJLU09YXCIsXG4gICAgc2hvcnRfbmFtZTogXCJLU09YXCIsXG4gICAgZGVzY3JpcHRpb246IFwiS1NPWCAtIENyeXB0b2N1cnJlbmN5IEV4Y2hhbmdlICYgUGF5bWVudCBQcm9jZXNzb3JcIixcbiAgICBpY29uczogW1xuICAgICAge1xuICAgICAgICBzcmM6IFwiL3B3YS9tc3RpbGUtMTUweDE1MC1kYXJrYmcucG5nXCIsXG4gICAgICAgIHNpemVzOiBcIjE1MHgxNTBcIixcbiAgICAgICAgdHlwZTogXCJpbWFnZS9wbmdcIixcbiAgICAgICAgcHVycG9zZTogXCJtYXNrYWJsZVwiLFxuICAgICAgfSxcbiAgICAgIHtcbiAgICAgICAgc3JjOiBcIi9wd2EvYW5kcm9pZC1jaHJvbWUtMTkyeDE5Mi5wbmdcIixcbiAgICAgICAgc2l6ZXM6IFwiMTkyeDE5MlwiLFxuICAgICAgICB0eXBlOiBcImltYWdlL3BuZ1wiLFxuICAgICAgfSxcbiAgICAgIHtcbiAgICAgICAgc3JjOiBcIi9wd2EvYW5kcm9pZC1jaHJvbWUtNTEyeDUxMi5wbmdcIixcbiAgICAgICAgc2l6ZXM6IFwiNTEyeDUxMlwiLFxuICAgICAgICB0eXBlOiBcImltYWdlL3BuZ1wiLFxuICAgICAgfSxcbiAgICBdLFxuICB9LFxufTtcblxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcbiAgcGx1Z2luczogW3NvbGlkKCksIFZpdGVQV0EocHdhT3B0aW9ucyldLFxuICBlbnZEaXI6IFwiLi4vLi4vXCIsXG4gIHNlcnZlcjoge1xuICAgIHByb3h5OiB7XG4gICAgICBcIi9hcGlcIjoge1xuICAgICAgICB0YXJnZXQ6IFwiaHR0cDovL2xvY2FsaG9zdDo4MDgwL1wiLFxuICAgICAgICBjaGFuZ2VPcmlnaW46IHRydWUsXG4gICAgICAgIHJld3JpdGU6IChwYXRoKSA9PiBwYXRoLnJlcGxhY2UoL15cXC9hcGkvLCBcIlwiKSxcbiAgICAgIH0sXG4gICAgfSxcbiAgfSxcbn0pO1xuIl0sCiAgIm1hcHBpbmdzIjogIjtBQUE0VyxTQUFTLG9CQUFvQjtBQUN6WSxTQUFTLGVBQW9DO0FBQzdDLE9BQU8sV0FBVztBQUNsQixTQUFTLG1CQUFtQjtBQUU1QixJQUFNLFlBQVksWUFBWSxFQUFFLEVBQUUsU0FBUyxLQUFLO0FBRWhELElBQU0sYUFBc0M7QUFBQSxFQUMxQyxjQUFjO0FBQUEsRUFDZCxTQUFTO0FBQUEsSUFDUCxjQUFjLENBQUMsb0NBQW9DO0FBQUEsSUFDbkQsa0JBQWtCO0FBQUEsSUFDbEIsdUJBQXVCO0FBQUEsSUFDdkIsMkJBQTJCLENBQUMsRUFBRSxVQUFVLFdBQVcsS0FBSyxJQUFJLENBQUM7QUFBQSxFQUMvRDtBQUFBLEVBQ0EsVUFBVTtBQUFBLElBQ1IsYUFBYTtBQUFBLElBQ2IsTUFBTTtBQUFBLElBQ04sWUFBWTtBQUFBLElBQ1osYUFBYTtBQUFBLElBQ2IsT0FBTztBQUFBLE1BQ0w7QUFBQSxRQUNFLEtBQUs7QUFBQSxRQUNMLE9BQU87QUFBQSxRQUNQLE1BQU07QUFBQSxRQUNOLFNBQVM7QUFBQSxNQUNYO0FBQUEsTUFDQTtBQUFBLFFBQ0UsS0FBSztBQUFBLFFBQ0wsT0FBTztBQUFBLFFBQ1AsTUFBTTtBQUFBLE1BQ1I7QUFBQSxNQUNBO0FBQUEsUUFDRSxLQUFLO0FBQUEsUUFDTCxPQUFPO0FBQUEsUUFDUCxNQUFNO0FBQUEsTUFDUjtBQUFBLElBQ0Y7QUFBQSxFQUNGO0FBQ0Y7QUFFQSxJQUFPLHNCQUFRLGFBQWE7QUFBQSxFQUMxQixTQUFTLENBQUMsTUFBTSxHQUFHLFFBQVEsVUFBVSxDQUFDO0FBQUEsRUFDdEMsUUFBUTtBQUFBLEVBQ1IsUUFBUTtBQUFBLElBQ04sT0FBTztBQUFBLE1BQ0wsUUFBUTtBQUFBLFFBQ04sUUFBUTtBQUFBLFFBQ1IsY0FBYztBQUFBLFFBQ2QsU0FBUyxDQUFDLFNBQVMsS0FBSyxRQUFRLFVBQVUsRUFBRTtBQUFBLE1BQzlDO0FBQUEsSUFDRjtBQUFBLEVBQ0Y7QUFDRixDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
