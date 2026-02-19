<script setup lang="ts">
interface Meeting {
  id: string;
  room_name: string;
  title: string | null;
  invite_url: string;
  created_at: string;
}

interface MeetingDetail {
  id: string;
  room_name: string;
  title: string | null;
  jitsi_jwt: string;
  invite_url: string;
  created_at: string;
}

const props = defineProps<{ meeting: Meeting }>();

const api = useApi();
const joining = ref(false);

function formatDate(iso: string) {
  return new Date(iso).toLocaleString("ru-RU", {
    day: "numeric",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

async function joinMeeting() {
  joining.value = true;
  try {
    const data = await api<MeetingDetail>(
      `/api/meetings/${props.meeting.id}`,
    );
    navigateTo(`/meeting/${data.room_name}?jwt=${data.jitsi_jwt}`);
  } catch {
    joining.value = false;
  }
}
</script>

<template>
  <UCard>
    <div class="flex items-start justify-between gap-4">
      <div class="min-w-0 flex-1">
        <h3 class="font-semibold text-lg truncate">
          {{ meeting.title || meeting.room_name }}
        </h3>
        <p class="text-sm text-gray-500 dark:text-gray-400 mt-0.5">
          {{ formatDate(meeting.created_at) }}
        </p>
      </div>
      <div class="flex gap-2 shrink-0">
        <InviteLink :invite-url="meeting.invite_url" />
        <UButton
          icon="i-lucide-video"
          size="sm"
          :loading="joining"
          @click="joinMeeting"
        >
          Войти
        </UButton>
      </div>
    </div>
  </UCard>
</template>
