interface JitsiMeetExternalAPIOptions {
  roomName: string
  jwt?: string
  parentNode?: HTMLElement
  width?: string | number
  height?: string | number
  configOverwrite?: Record<string, unknown>
  interfaceConfigOverwrite?: Record<string, unknown>
  lang?: string
  onload?: () => void
}

declare class JitsiMeetExternalAPI {
  constructor(domain: string, options?: JitsiMeetExternalAPIOptions)
  dispose(): void
  addListener(event: string, listener: (...args: unknown[]) => void): void
  removeListener(event: string, listener: (...args: unknown[]) => void): void
  executeCommand(command: string, ...args: unknown[]): void
}

interface Window {
  JitsiMeetExternalAPI: typeof JitsiMeetExternalAPI | undefined
}
