export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  future: {
    compatibilityVersion: 4,
  },
  typescript: {
    strict: true,
  },
  modules: ["@nuxt/ui", "@pinia/nuxt"],
  runtimeConfig: {
    public: {
      apiBase: "http://localhost:8080",
      jitsiDomain: "localhost:8443",
    },
  },
});
