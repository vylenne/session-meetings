<script setup lang="ts">
definePageMeta({ layout: false })

interface MeetingResponse {
  id: string
  room_name: string
  title: string | null
  jitsi_jwt: string
  invite_url: string
  created_at: string
}

const route = useRoute()
const config = useRuntimeConfig()
const api = useApi()

const room = computed(() => route.params.room as string)
const jitsiJwt = ref('')
const loading = ref(true)
const error = ref('')

onMounted(async () => {
  const queryJwt = route.query.jwt as string | undefined
  if (queryJwt) {
    jitsiJwt.value = queryJwt
    window.history.replaceState({}, '', `/meeting/${room.value}`)
    loading.value = false
    return
  }

  try {
    const data = await api<MeetingResponse>(
      `/api/meetings/room/${room.value}`,
    )
    jitsiJwt.value = data.jitsi_jwt
  } catch {
    error.value = 'Не удалось подключиться к встрече'
  } finally {
    loading.value = false
  }
})

function onReadyToClose() {
  navigateTo('/dashboard')
}

function onJitsiError(message: string) {
  error.value = message
}
</script>

<template>
  <div class="fixed inset-0 bg-gray-950">
    <div v-if="loading" class="flex items-center justify-center h-full">
      <div class="text-center">
        <UIcon name="i-lucide-loader-2" class="animate-spin text-4xl text-white mb-4" />
        <p class="text-gray-400">Подключение к встрече...</p>
      </div>
    </div>

    <div v-else-if="error" class="flex items-center justify-center h-full">
      <UCard class="max-w-md">
        <div class="text-center space-y-4">
          <UIcon name="i-lucide-video-off" class="text-5xl text-red-400" />
          <p class="text-lg font-medium">{{ error }}</p>
          <UButton to="/dashboard" icon="i-lucide-arrow-left">
            Вернуться на главную
          </UButton>
        </div>
      </UCard>
    </div>

    <template v-else>
      <JitsiMeet
        :room-name="room"
        :jwt="jitsiJwt"
        :domain="config.public.jitsiDomain as string"
        @ready-to-close="onReadyToClose"
        @error="onJitsiError"
      />
      <button
        class="absolute top-4 left-4 z-10 bg-gray-900/80 hover:bg-gray-800 text-white rounded-full p-2 backdrop-blur-sm transition-colors"
        title="Вернуться на главную"
        @click="navigateTo('/dashboard')"
      >
        <UIcon name="i-lucide-arrow-left" class="text-xl" />
      </button>
    </template>
  </div>
</template>
