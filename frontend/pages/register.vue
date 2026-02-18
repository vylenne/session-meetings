<script setup lang="ts">
definePageMeta({ layout: false });

const { register, isAuthenticated } = useAuth();
const route = useRoute();

const form = reactive({ name: "", email: "", password: "", confirmPassword: "" });
const loading = ref(false);
const error = ref("");

if (isAuthenticated.value) {
  navigateTo((route.query.redirect as string) || "/dashboard");
}

async function onSubmit() {
  error.value = "";

  if (form.password !== form.confirmPassword) {
    error.value = "Пароли не совпадают";
    return;
  }

  if (form.password.length < 8) {
    error.value = "Пароль должен быть минимум 8 символов";
    return;
  }

  loading.value = true;
  try {
    await register({ name: form.name, email: form.email, password: form.password });
    navigateTo((route.query.redirect as string) || "/dashboard");
  } catch (e: unknown) {
    const msg =
      e && typeof e === "object" && "data" in e
        ? String((e as Record<string, unknown>).data)
        : "Ошибка регистрации";
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
          <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">Создайте аккаунт</p>
        </div>
      </template>

      <form class="space-y-4" @submit.prevent="onSubmit">
        <UFormField label="Имя">
          <UInput
            v-model="form.name"
            placeholder="Ваше имя"
            required
            icon="i-lucide-user"
            size="lg"
          />
        </UFormField>

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

        <UFormField label="Подтверждение пароля">
          <UInput
            v-model="form.confirmPassword"
            type="password"
            placeholder="Повторите пароль"
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
          Зарегистрироваться
        </UButton>
      </form>

      <template #footer>
        <p class="text-center text-sm text-gray-500 dark:text-gray-400">
          Уже есть аккаунт?
          <NuxtLink to="/login" class="text-primary font-medium hover:underline">
            Войти
          </NuxtLink>
        </p>
      </template>
    </UCard>
  </div>
</template>
