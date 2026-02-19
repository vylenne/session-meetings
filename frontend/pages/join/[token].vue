<script setup lang="ts">
definePageMeta({ layout: false })

interface InviteSuccess {
  room_name: string
  jitsi_jwt: string
}

interface InviteRedirect {
  redirect: string
}

type InviteResult = InviteSuccess | InviteRedirect

const route = useRoute()
const api = useApi()
const token = computed(() => route.params.token as string)

const errorMessage = ref('')

const { data, error } = await useAsyncData(
  `invite-${token.value}`,
  () => api<InviteResult>(`/api/invite/${token.value}`),
)

if (data.value) {
  if ('room_name' in data.value && 'jitsi_jwt' in data.value) {
    await navigateTo(
      `/meeting/${data.value.room_name}?jwt=${data.value.jitsi_jwt}`,
      { replace: true },
    )
  } else if ('redirect' in data.value) {
    await navigateTo(data.value.redirect, { replace: true })
  }
} else if (error.value) {
  const statusCode = error.value.statusCode ?? 0
  if (statusCode === 404) {
    errorMessage.value = 'Ссылка-приглашение недействительна'
  } else if (statusCode === 400) {
    errorMessage.value = 'Срок действия приглашения истёк'
  } else {
    errorMessage.value = 'Произошла ошибка при проверке приглашения'
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-950 px-4">
    <div v-if="!errorMessage" class="text-center">
      <UIcon name="i-lucide-loader-2" class="animate-spin text-4xl text-gray-400 mb-4" />
      <p class="text-gray-500 dark:text-gray-400">Проверка приглашения...</p>
    </div>

    <UCard v-else class="max-w-md w-full">
      <div class="text-center space-y-4">
        <UIcon name="i-lucide-link-2-off" class="text-5xl text-red-400" />
        <h2 class="text-xl font-semibold">{{ errorMessage }}</h2>
        <p class="text-gray-500 dark:text-gray-400 text-sm">
          Попросите организатора отправить новую ссылку
        </p>
        <div class="flex justify-center gap-3">
          <UButton to="/login" variant="soft" icon="i-lucide-log-in">
            Войти
          </UButton>
          <UButton to="/dashboard" icon="i-lucide-layout-dashboard">
            На главную
          </UButton>
        </div>
      </div>
    </UCard>
  </div>
</template>
