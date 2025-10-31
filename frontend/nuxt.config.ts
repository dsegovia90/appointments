// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: "2025-07-15",
  app: {
    pageTransition: { name: "page", mode: "out-in" },
  },
  devtools: { enabled: true },
  modules: ["@nuxt/eslint", "@nuxt/ui", "@pinia/nuxt", "@vueuse/nuxt"],
  css: ["~/assets/css/main.css"],
  vite: {
    server: {
      proxy: {
        "/api": {
          target: "http://localhost:5150",
          changeOrigin: true,
        },
      },
    },
  },
  nitro: {
    compressPublicAssets: true,
  },
});
