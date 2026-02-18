export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig();
  const authCookie = useCookie("auth_token");

  const api = $fetch.create({
    baseURL: config.public.apiBase as string,
    onRequest({ options }) {
      if (authCookie.value) {
        const headers = new Headers(options.headers);
        headers.set("Authorization", `Bearer ${authCookie.value}`);
        options.headers = headers;
      }
    },
    onResponseError({ response }) {
      if (response.status === 401) {
        authCookie.value = null;
        navigateTo("/login");
      }
    },
  });

  return {
    provide: {
      api,
    },
  };
});
