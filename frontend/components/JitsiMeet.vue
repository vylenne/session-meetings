<script setup lang="ts">
const props = defineProps<{
  roomName: string
  jwt: string
  domain: string
}>()

const emit = defineEmits<{
  'ready-to-close': []
  'error': [message: string]
}>()

const containerRef = ref<HTMLDivElement | null>(null)
let jitsiApi: JitsiMeetExternalAPI | null = null

function loadScript(src: string): Promise<void> {
  return new Promise((resolve, reject) => {
    if (typeof window.JitsiMeetExternalAPI !== 'undefined') {
      resolve()
      return
    }
    const existing = document.querySelector(`script[src="${src}"]`)
    if (existing) existing.remove()

    const el = document.createElement('script')
    el.src = src
    el.async = true
    el.onload = () => resolve()
    el.onerror = () => reject(new Error(`Failed to load ${src}`))
    document.head.appendChild(el)
  })
}

onMounted(async () => {
  if (!containerRef.value) return

  try {
    await loadScript(`https://${props.domain}/external_api.js`)
  } catch {
    emit('error', `Не удалось загрузить Jitsi с ${props.domain}. Убедитесь, что сертификат принят в браузере.`)
    return
  }

  try {
    jitsiApi = new JitsiMeetExternalAPI(props.domain, {
      roomName: props.roomName,
      jwt: props.jwt,
      parentNode: containerRef.value,
      width: '100%',
      height: '100%',
      configOverwrite: {
        disableDeepLinking: true,
        e2eeMode: 'optional',
        prejoinPageEnabled: true,
      },
      interfaceConfigOverwrite: {
        TOOLBAR_BUTTONS: [
          'microphone', 'camera', 'desktop', 'chat',
          'raisehand', 'participants-pane', 'tileview', 'security',
        ],
      },
    })
  } catch (e) {
    emit('error', `Ошибка инициализации Jitsi: ${e instanceof Error ? e.message : 'неизвестная ошибка'}`)
    return
  }

  const iframe = containerRef.value.querySelector('iframe')
  if (iframe) {
    iframe.allow = 'camera; microphone; display-capture; autoplay; clipboard-write'
  }

  jitsiApi.addListener('readyToClose', () => {
    emit('ready-to-close')
  })

  jitsiApi.addListener('videoConferenceLeft', () => {
    emit('ready-to-close')
  })

  jitsiApi.addListener('errorOccurred', (...args: unknown[]) => {
    const payload = args[0] as { error?: { name?: string; message?: string } } | undefined
    const err = payload?.error
    emit('error', err?.message || err?.name || 'Произошла ошибка соединения')
  })
})

onBeforeUnmount(() => {
  if (jitsiApi) {
    jitsiApi.dispose()
    jitsiApi = null
  }
})
</script>

<template>
  <div ref="containerRef" class="w-full h-full" />
</template>
