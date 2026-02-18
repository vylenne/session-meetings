import { useAuthStore } from "~/stores/auth";

interface LoginPayload {
  email: string;
  password: string;
}

interface RegisterPayload {
  email: string;
  password: string;
  name: string;
}

interface AuthResponse {
  token: string;
}

export function useAuth() {
  const store = useAuthStore();
  const api = useApi();

  async function login(payload: LoginPayload) {
    const data = await api<AuthResponse>("/api/auth/login", {
      method: "POST",
      body: payload,
    });
    store.setToken(data.token);
    await store.fetchUser();
  }

  async function register(payload: RegisterPayload) {
    const data = await api<AuthResponse>("/api/auth/register", {
      method: "POST",
      body: payload,
    });
    store.setToken(data.token);
    await store.fetchUser();
  }

  return {
    login,
    register,
    logout: store.logout,
    user: computed(() => store.user),
    isAuthenticated: computed(() => store.isAuthenticated),
    fetchUser: store.fetchUser,
  };
}
