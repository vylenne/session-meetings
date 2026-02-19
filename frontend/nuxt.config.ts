export default defineNuxtConfig({
  compatibilityDate: "2024-11-01",
  future: {
    compatibilityVersion: 4,
  },
  typescript: {
    strict: true,
  },
  css: ["~/assets/css/main.css"],
  modules: ["@nuxt/ui", "@pinia/nuxt"],
  routeRules: {
    "/meeting/**": {
      headers: {
        "Permissions-Policy":
          "camera=*, microphone=*, display-capture=*",
      },
    },
  },
  runtimeConfig: {
    apiBaseInternal: "http://localhost:8080",
    public: {
      apiBase: "http://localhost:8080",
      jitsiDomain: "localhost:8443",
    },
  },
});
