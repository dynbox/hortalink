import { defineConfig } from "astro/config";
import node from "@astrojs/node";

// https://astro.build/config
export default defineConfig({
  output: "server",
  adapter: node({
    mode: "standalone"
  }),
  server: {
    headers: {
      'Access-Control-Allow-Origin': "http://localhost:4321" // isso aqui vai ser dinâmico
    }
  }
});