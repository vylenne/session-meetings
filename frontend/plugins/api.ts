export default defineNuxtPlugin(() => {
  const config = useRuntimeConfig();
  const authCookie = useCookie("auth_token");

  const baseURL = import.meta.server
    ? (config.apiBaseInternal as string)
    : (config.public.apiBase as string);

  const api = $fetch.create({
    baseURL,
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
