import { useAuthStore } from "~/stores/auth";

const publicRoutes = ["/login", "/register", "/", "/join"];

export default defineNuxtRouteMiddleware(async (to) => {
  const store = useAuthStore();

  const isPublic = publicRoutes.some(
    (route) => to.path === route || to.path.startsWith("/join/"),
  );

  if (isPublic) return;

  if (!store.isAuthenticated) {
    return navigateTo(`/login?redirect=${encodeURIComponent(to.fullPath)}`);
  }

  if (!store.user) {
    await store.fetchUser();
  }
});
