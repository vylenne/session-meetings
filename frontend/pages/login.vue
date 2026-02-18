<script setup lang="ts">
definePageMeta({ layout: false });

const { login, isAuthenticated } = useAuth();
const route = useRoute();

const form = reactive({ email: "", password: "" });
const loading = ref(false);
const error = ref("");

if (isAuthenticated.value) {
  navigateTo((route.query.redirect as string) || "/dashboard");
}

async function onSubmit() {
  error.value = "";
  loading.value = true;
  try {
    await login({ email: form.email, password: form.password });
    navigateTo((route.query.redirect as string) || "/dashboard");
  } catch (e: unknown) {
    const msg =
      e && typeof e === "object" && "data" in e
        ? String((e as Record<string, unknown>).data)
        : "Неверный email или пароль";
    error.value = msg;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-950 px-4">
    <UCard class="w-full max-w-md">
      <template #header>
        <div class="text-center">
          <h1 class="text-2xl font-bold">Session Meeting</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Войдите в аккаунт</p>
        </div>
      </template>

      <form class="space-y-4" @submit.prevent="onSubmit">
        <UFormField label="Email">
          <UInput
            v-model="form.email"
            type="email"
            placeholder="you@example.com"
            required
            icon="i-lucide-mail"
            size="lg"
          />
        </UFormField>

        <UFormField label="Пароль">
          <UInput
            v-model="form.password"
            type="password"
            placeholder="Минимум 8 символов"
            required
            icon="i-lucide-lock"
            size="lg"
          />
        </UFormField>

        <UAlert
          v-if="error"
          color="error"
          variant="subtle"
          :title="error"
          icon="i-lucide-circle-alert"
        />

        <UButton type="submit" block size="lg" :loading="loading">
          Войти
        </UButton>
      </form>

      <template #footer>
        <p class="text-center text-sm text-gray-500 dark:text-gray-400">
          Нет аккаунта?
          <NuxtLink to="/register" class="text-primary font-medium hover:underline">
            Зарегистрироваться
          </NuxtLink>
        </p>
      </template>
    </UCard>
  </div>
</template>
