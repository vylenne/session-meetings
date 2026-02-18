<script setup lang="ts">
definePageMeta({});

interface Meeting {
  id: string;
  room_name: string;
  title: string | null;
  invite_url: string;
  created_at: string;
}

interface CreateMeetingResponse {
  id: string;
  room_name: string;
  title: string | null;
  jitsi_jwt: string;
  invite_url: string;
  created_at: string;
}

const api = useApi();
const { user, logout } = useAuth();

const meetings = ref<Meeting[]>([]);
const loading = ref(true);
const creating = ref(false);
const showNewMeeting = ref(false);
const newTitle = ref("");

async function fetchMeetings() {
  loading.value = true;
  try {
    meetings.value = await api<Meeting[]>("/api/meetings");
  } catch {
    meetings.value = [];
  } finally {
    loading.value = false;
  }
}

async function createMeeting() {
  creating.value = true;
  try {
    const data = await api<CreateMeetingResponse>("/api/meetings", {
      method: "POST",
      body: { title: newTitle.value || null },
    });
    showNewMeeting.value = false;
    newTitle.value = "";
    navigateTo(`/meeting/${data.room_name}?jwt=${data.jitsi_jwt}`);
  } catch {
    creating.value = false;
  }
}

onMounted(fetchMeetings);
</script>

<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-950">
    <header class="border-b border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900">
      <div class="max-w-5xl mx-auto px-4 py-4 flex items-center justify-between">
        <NuxtLink to="/dashboard" class="text-xl font-bold">Session Meeting</NuxtLink>
        <div class="flex items-center gap-3">
          <span class="text-sm text-gray-500 dark:text-gray-400">{{ user?.name }}</span>
          <UButton variant="ghost" icon="i-lucide-log-out" size="sm" @click="logout">
            Выйти
          </UButton>
        </div>
      </div>
    </header>

    <main class="max-w-5xl mx-auto px-4 py-8">
      <div class="flex items-center justify-between mb-6">
        <h1 class="text-2xl font-bold">Мои встречи</h1>
        <UButton icon="i-lucide-plus" size="lg" @click="showNewMeeting = true">
          Новая встреча
        </UButton>
      </div>

      <UModal v-model:open="showNewMeeting">
        <template #content>
          <UCard>
            <template #header>
              <h2 class="text-lg font-semibold">Создать встречу</h2>
            </template>
            <form class="space-y-4" @submit.prevent="createMeeting">
              <UFormField label="Название (необязательно)">
                <UInput
                  v-model="newTitle"
                  placeholder="Например: Стендап команды"
                  icon="i-lucide-text"
                  size="lg"
                />
              </UFormField>
              <div class="flex justify-end gap-2">
                <UButton variant="ghost" @click="showNewMeeting = false">Отмена</UButton>
                <UButton type="submit" :loading="creating" icon="i-lucide-video">
                  Создать и войти
                </UButton>
              </div>
            </form>
          </UCard>
        </template>
      </UModal>

      <div v-if="loading" class="flex justify-center py-16">
        <UIcon name="i-lucide-loader-2" class="animate-spin text-3xl text-gray-400" />
      </div>

      <div v-else-if="meetings.length === 0" class="text-center py-16">
        <UIcon name="i-lucide-video-off" class="text-5xl text-gray-300 dark:text-gray-600 mb-4" />
        <p class="text-gray-500 dark:text-gray-400 text-lg">Пока нет встреч</p>
        <p class="text-gray-400 dark:text-gray-500 text-sm mt-1">
          Нажмите «Новая встреча» чтобы начать
        </p>
      </div>

      <div v-else class="space-y-3">
        <MeetingCard v-for="m in meetings" :key="m.id" :meeting="m" />
      </div>
    </main>
  </div>
</template>
