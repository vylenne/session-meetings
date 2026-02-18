<script setup lang="ts">
definePageMeta({ layout: false });

const { isAuthenticated } = useAuth();

if (import.meta.client && isAuthenticated.value) {
  navigateTo("/dashboard");
}
</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-gray-50 dark:bg-gray-950 px-4">
    <div class="text-center max-w-lg">
      <h1 class="text-4xl font-bold mb-3">Session Meeting</h1>
      <p class="text-lg text-gray-500 dark:text-gray-400 mb-8">
        Безопасные видеозвонки с шифрованием на базе Jitsi Meet
      </p>
      <div class="flex gap-3 justify-center">
        <UButton to="/login" size="lg" variant="solid">
          Войти
        </UButton>
        <UButton to="/register" size="lg" variant="outline">
          Регистрация
        </UButton>
      </div>
    </div>
  </div>
</template>
