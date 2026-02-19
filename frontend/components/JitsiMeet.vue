<script setup lang="ts">
const props = defineProps<{
  roomName: string
  jwt: string
  domain: string
}>()

const emit = defineEmits<{
  'ready-to-close': []
}>()

const containerRef = ref<HTMLDivElement | null>(null)
let jitsiApi: JitsiMeetExternalAPI | null = null

function loadScript(src: string): Promise<void> {
  return new Promise((resolve, reject) => {
    if (document.querySelector(`script[src="${src}"]`)) {
      resolve()
      return
    }
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

  await loadScript(`https://${props.domain}/external_api.js`)

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

  const iframe = containerRef.value.querySelector('iframe')
  if (iframe) {
    iframe.allow = 'camera; microphone; display-capture; autoplay; clipboard-write'
  }

  jitsiApi.addListener('readyToClose', () => {
    emit('ready-to-close')
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
