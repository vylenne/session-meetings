import { defineStore } from "pinia";

interface User {
  id: string;
  email: string;
  name: string;
}

export const useAuthStore = defineStore("auth", () => {
  const user = ref<User | null>(null);
  const token = useCookie("auth_token", { maxAge: 60 * 60 * 24 });
  const isAuthenticated = computed(() => !!token.value);

  function setToken(newToken: string) {
    token.value = newToken;
  }

  function logout() {
    token.value = null;
    user.value = null;
    navigateTo("/login");
  }

  async function fetchUser() {
    if (!token.value) return;
    try {
      const { $api } = useNuxtApp();
      user.value = await $api<User>("/api/auth/me");
    } catch {
      logout();
    }
  }

  return { user, token, isAuthenticated, setToken, logout, fetchUser };
});
