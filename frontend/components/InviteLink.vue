<script setup lang="ts">
const props = defineProps<{
  inviteUrl: string
}>()

const copied = ref(false)
const toast = useToast()

async function copyLink() {
  try {
    const url = `${window.location.origin}${props.inviteUrl}`
    await navigator.clipboard.writeText(url)
    copied.value = true
    toast.add({ title: 'Ссылка скопирована', color: 'success' })
    setTimeout(() => (copied.value = false), 2000)
  } catch {
    toast.add({ title: 'Не удалось скопировать', color: 'error' })
  }
}
</script>

<template>
  <UButton
    :icon="copied ? 'i-lucide-check' : 'i-lucide-link'"
    variant="soft"
    size="sm"
    @click="copyLink"
  >
    {{ copied ? 'Скопировано' : 'Ссылка' }}
  </UButton>
</template>
