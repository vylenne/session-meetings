<script setup lang="ts">
interface Meeting {
  id: string;
  room_name: string;
  title: string | null;
  invite_url: string;
  created_at: string;
}

const props = defineProps<{ meeting: Meeting }>();

const copied = ref(false);
const toast = useToast();

function formatDate(iso: string) {
  return new Date(iso).toLocaleString("ru-RU", {
    day: "numeric",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

async function copyInvite() {
  try {
    const url = `${window.location.origin}/join/${props.meeting.invite_url}`;
    await navigator.clipboard.writeText(url);
    copied.value = true;
    toast.add({ title: "Ссылка скопирована", color: "success" });
    setTimeout(() => (copied.value = false), 2000);
  } catch {
    toast.add({ title: "Не удалось скопировать", color: "error" });
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
        <UButton
          :icon="copied ? 'i-lucide-check' : 'i-lucide-link'"
          variant="soft"
          size="sm"
          @click="copyInvite"
        >
          {{ copied ? "Скопировано" : "Ссылка" }}
        </UButton>
        <UButton
          icon="i-lucide-video"
          size="sm"
          :to="`/meeting/${meeting.room_name}`"
        >
          Войти
        </UButton>
      </div>
    </div>
  </UCard>
</template>
