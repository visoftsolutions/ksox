/// <reference types="unlighthouse" />
import { defineConfig } from "unlighthouse";

export default defineConfig({
  site: "http://127.0.0.1/",
  debug: true,
  scanner: {
    device: "desktop",
    // device: 'mobile',
  },
});
