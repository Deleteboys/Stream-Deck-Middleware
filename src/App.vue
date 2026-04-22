<template>
  <v-app class="app-background">
    <v-app-bar flat color="transparent" class="px-4 pt-2">
      <v-icon size="x-large" color="primary" class="mr-3">mdi-keyboard-variant</v-icon>
      <div class="text-h5 font-weight-black text-white letter-spacing-tight">DECK<span class="text-primary">CONTROL</span></div>
      <v-spacer></v-spacer>
      <div class="d-flex align-center bg-surface-variant rounded-pill px-4 py-1 border-subtle">
        <div :class="['status-dot', picoConnected ? 'bg-success' : 'bg-error']"></div>
        <span class="text-caption font-weight-bold ml-2 text-grey-lighten-1">
          {{ picoConnected ? 'Pico Online' : 'Suche Gerät...' }}
        </span>
      </div>
    </v-app-bar>

    <v-main>
      <v-container class="fill-height d-flex flex-column align-center justify-center relative">
        <div class="text-caption text-grey-darken-1 mb-6 text-uppercase tracking-widest">
          Linksklick: Auslösen • Rechtsklick: Konfigurieren
        </div>

        <div class="deck-grid">
          <v-hover v-for="btn in buttonConfigs" :key="btn.id" v-slot="{ isHovering, props }">
            <div
                v-bind="props"
                :class="['deck-key', isHovering ? 'hovered' : '', editingBtnId === btn.id ? 'editing' : '']"
                @click="triggerAction(btn.command)"
                @contextmenu.prevent="openConfig(btn)"
            >
              <div class="key-content">
                <v-icon :color="btn.command === 'StartBootloader' ? 'error' : (editingBtnId === btn.id ? 'primary' : 'white')" size="42" class="mb-3 drop-shadow">
                  {{ btn.icon }}
                </v-icon>
                <div :class="['key-label', btn.command === 'StartBootloader' ? 'text-error' : '']">{{ btn.label }}</div>
              </div>
            </div>
          </v-hover>
        </div>

        <div class="log-container mt-12 w-100" style="max-width: 800px;">
          <div class="d-flex align-center mb-2">
            <v-icon size="x-small" color="grey" class="mr-2">mdi-console-line</v-icon>
            <span class="text-overline text-grey">System Log</span>
          </div>
          <div class="log-window">
            <div v-for="(log, idx) in picoLogs" :key="idx" class="log-entry">
              <span class="text-primary mr-2">❯</span>{{ log }}
            </div>
            <div v-if="picoLogs.length === 0" class="text-grey text-caption font-italic">
              Warte auf serielle Kommunikation...
            </div>
          </div>
        </div>
      </v-container>
    </v-main>

    <v-navigation-drawer
        v-model="isConfigOpen"
        location="right"
        width="400"
        temporary
        color="#121212"
        elevation="24"
        class="border-left"
    >
      <div v-if="editData" class="pa-6 d-flex flex-column fill-height">
        <div class="d-flex justify-space-between align-center mb-8">
          <div class="text-h6 font-weight-bold">Tasten-Setup</div>
          <v-btn icon="mdi-close" variant="text" density="comfortable" @click="isConfigOpen = false"></v-btn>
        </div>

        <v-text-field v-model="editData.label" label="Beschriftung" variant="underlined" color="primary" class="mb-2"></v-text-field>
        <v-text-field v-model="editData.icon" label="MDI Icon Name" variant="underlined" color="primary" class="mb-6"></v-text-field>

        <div class="text-subtitle-2 text-grey mb-2 text-uppercase tracking-widest">Aktion / Effekt</div>
        <v-select
            v-model="editData.effectName"
            :items="effectNames"
            variant="outlined"
            density="comfortable"
            bg-color="#1A1A1A"
            color="primary"
            class="mb-6"
        ></v-select>

        <v-slide-y-transition group>
          <div v-if="currentEffectNeeds.includes('color')" key="color" class="mb-6">
            <div class="text-caption text-grey mb-2">Basisfarbe</div>
            <v-color-picker
                v-model="editData.color"
                mode="rgb"
                hide-inputs
                show-swatches
                width="100%"
                elevation="0"
                bg-color="transparent"
            ></v-color-picker>
          </div>

          <div v-if="currentEffectNeeds.includes('brightness')" key="bright" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Helligkeit</span><span>{{ Math.round(editData.params.brightness) }}</span>
            </div>
            <v-slider v-model="editData.params.brightness" min="0" max="255" step="1" color="white" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('speed')" key="speed" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Tempo</span><span>{{ Math.round(editData.params.speed) }}</span>
            </div>
            <v-slider v-model="editData.params.speed" min="1" max="255" step="1" color="primary" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('size')" key="size" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Größe</span><span>{{ Math.round(editData.params.size) }}</span>
            </div>
            <v-slider v-model="editData.params.size" min="1" max="20" step="1" color="secondary" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('tail')" key="tail" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Schweif-Länge</span><span>{{ Math.round(editData.params.tail) }}</span>
            </div>
            <v-slider v-model="editData.params.tail" min="1" max="20" step="1" color="deep-orange" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('density')" key="density" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Dichte</span><span>{{ Math.round(editData.params.density) }}</span>
            </div>
            <v-slider v-model="editData.params.density" min="1" max="255" step="1" color="pink" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('hue')" key="hue" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Start-Farbton (Hue)</span><span>{{ Math.round(editData.params.hue) }}</span>
            </div>
            <v-slider v-model="editData.params.hue" min="0" max="255" step="1" color="purple" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('hue_shift')" key="hue_shift" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Farb-Verschiebung</span><span>{{ Math.round(editData.params.hue_shift) }}</span>
            </div>
            <v-slider v-model="editData.params.hue_shift" min="1" max="100" step="1" color="indigo" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('saturation')" key="saturation" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Sättigung</span><span>{{ Math.round(editData.params.saturation) }}</span>
            </div>
            <v-slider v-model="editData.params.saturation" min="0" max="255" step="1" color="blue" track-color="#333" hide-details></v-slider>
          </div>

          <div v-if="currentEffectNeeds.includes('spread')" key="spread" class="mb-4">
            <div class="d-flex justify-space-between text-caption text-grey mb-1">
              <span>Farb-Spreizung</span><span>{{ Math.round(editData.params.spread) }}</span>
            </div>
            <v-slider v-model="editData.params.spread" min="1" max="255" step="1" color="teal" track-color="#333" hide-details></v-slider>
          </div>
        </v-slide-y-transition>

        <v-spacer></v-spacer>

        <v-btn block color="primary" size="x-large" class="mt-8 font-weight-bold" @click="saveSettings">
          Speichern & Anwenden
        </v-btn>
      </div>
    </v-navigation-drawer>
  </v-app>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

interface ButtonConfig { id: number; label: string; icon: string; command: any; }

const picoConnected = ref(false)
const picoLogs = ref<string[]>([])
let unlistenFn: UnlistenFn | null = null

const buttonConfigs = ref<ButtonConfig[]>([
  { id: 1, label: 'Solid White', icon: 'mdi-lightbulb-on', command: { FillAll: { r: 255, g: 255, b: 255, brightness: 50 } } },
  { id: 2, label: 'Blink Red', icon: 'mdi-alarm-light', command: { SetEffect: { effect: { Blink: { r: 255, g: 0, b: 0, brightness: 50, speed: 200 } } } } },
  { id: 3, label: 'Rainbow', icon: 'mdi-palette', command: { SetEffect: { effect: { Rainbow: { speed: 15, brightness: 40 } } } } },
  { id: 4, label: 'Breathe Blue', icon: 'mdi-weather-windy', command: { SetEffect: { effect: { Breathing: { r: 0, g: 150, b: 255, brightness: 50, speed: 5 } } } } },
  { id: 5, label: 'Chase Green', icon: 'mdi-run', command: { SetEffect: { effect: { Chase: { r: 0, g: 255, b: 0, brightness: 50, speed: 80, size: 3 } } } } },
  { id: 6, label: 'Comet Fire', icon: 'mdi-fire', command: { SetEffect: { effect: { Comet: { r: 255, g: 100, b: 0, brightness: 50, speed: 60, tail: 8 } } } } },
  { id: 7, label: 'Sparkle', icon: 'mdi-creation', command: { SetEffect: { effect: { Sparkle: { r: 255, g: 255, b: 255, brightness: 50, speed: 150, density: 50 } } } } },
  { id: 8, label: 'Aurora', icon: 'mdi-waves', command: { SetEffect: { effect: { Aurora: { speed: 10, brightness: 40 } } } } },
  { id: 9, label: 'Orbit', icon: 'mdi-sync', command: { SetEffect: { effect: { ColorOrbit: { hue: 140, hue_shift: 16, saturation: 220, brightness: 50, speed: 35 } } } } },
  { id: 10, label: 'Astolfo', icon: 'mdi-heart', command: { SetEffect: { effect: { Astolfo: { brightness: 50, speed: 190, saturation: 220, spread: 90 } } } } },
  { id: 11, label: 'Blackout', icon: 'mdi-power', command: { FillAll: { r: 0, g: 0, b: 0, brightness: 0 } } },
  { id: 12, label: 'Flash Mode', icon: 'mdi-cellphone-arrow-down', command: 'StartBootloader' },
])

const effectRequirements: Record<string, string[]> = {
  FillAll: ['color', 'brightness'],
  Solid: ['color', 'brightness'],
  Blink: ['color', 'brightness', 'speed'],
  Rainbow: ['brightness', 'speed'],
  Breathing: ['color', 'brightness', 'speed'],
  Chase: ['color', 'brightness', 'speed', 'size'],
  Comet: ['color', 'brightness', 'speed', 'tail'],
  Sparkle: ['color', 'brightness', 'speed', 'density'],
  Aurora: ['brightness', 'speed'],
  ColorOrbit: ['hue', 'hue_shift', 'saturation', 'brightness', 'speed'],
  Astolfo: ['brightness', 'speed', 'saturation', 'spread'],
  Ping: [],
  StartBootloader: []
}
const effectNames = Object.keys(effectRequirements)

const isConfigOpen = ref(false)
const editingBtnId = ref<number | null>(null)

const editData = ref({
  label: '',
  icon: '',
  effectName: 'FillAll',
  color: '#FFFFFF',
  params: {
    brightness: 50, speed: 100, size: 3, tail: 5, density: 128,
    hue: 140, hue_shift: 16, saturation: 220, spread: 90
  }
})

const currentEffectNeeds = computed(() => effectRequirements[editData.value.effectName] || [])

const rgbToHex = (r: number, g: number, b: number) => "#" + (1 << 24 | r << 16 | g << 8 | b).toString(16).slice(1).toUpperCase()
const hexToRgb = (hex: string) => {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  return result ? { r: parseInt(result[1], 16), g: parseInt(result[2], 16), b: parseInt(result[3], 16) } : { r: 255, g: 255, b: 255 }
}

const openConfig = (btn: ButtonConfig) => {
  editingBtnId.value = btn.id
  editData.value.label = btn.label
  editData.value.icon = btn.icon
  const cmd = btn.command

  if (cmd === 'Ping' || cmd === 'StartBootloader') {
    editData.value.effectName = cmd
  } else if (cmd.FillAll) {
    editData.value.effectName = 'FillAll'
    editData.value.color = rgbToHex(cmd.FillAll.r, cmd.FillAll.g, cmd.FillAll.b)
    editData.value.params.brightness = cmd.FillAll.brightness || 50
  } else if (cmd.SetEffect && cmd.SetEffect.effect) {
    const effectName = Object.keys(cmd.SetEffect.effect)[0]
    editData.value.effectName = effectName
    const p = cmd.SetEffect.effect[effectName]
    if (p.r !== undefined) editData.value.color = rgbToHex(p.r, p.g, p.b)
    Object.assign(editData.value.params, p)
  }
  isConfigOpen.value = true
}

const saveSettings = async () => {
  const btnIndex = buttonConfigs.value.findIndex(b => b.id === editingBtnId.value)
  if (btnIndex === -1) return

  const needs = currentEffectNeeds.value
  const p = editData.value.params
  const rgb = hexToRgb(editData.value.color)
  const effectName = editData.value.effectName

  let newCmd: any = {}

  if (effectName === 'Ping' || effectName === 'StartBootloader') {
    newCmd = effectName
  } else if (effectName === 'FillAll') {
    newCmd = { FillAll: { r: Math.round(rgb.r), g: Math.round(rgb.g), b: Math.round(rgb.b), brightness: Math.round(p.brightness) } }
  } else {
    let payload: any = {}
    if (needs.includes('color')) { payload.r = Math.round(rgb.r); payload.g = Math.round(rgb.g); payload.b = Math.round(rgb.b) }
    if (needs.includes('brightness')) payload.brightness = Math.round(p.brightness)
    if (needs.includes('speed')) payload.speed = Math.round(p.speed)
    if (needs.includes('size')) payload.size = Math.round(p.size)
    if (needs.includes('tail')) payload.tail = Math.round(p.tail)
    if (needs.includes('density')) payload.density = Math.round(p.density)
    if (needs.includes('hue')) payload.hue = Math.round(p.hue)
    if (needs.includes('hue_shift')) payload.hue_shift = Math.round(p.hue_shift)
    if (needs.includes('saturation')) payload.saturation = Math.round(p.saturation)
    if (needs.includes('spread')) payload.spread = Math.round(p.spread)

    newCmd = { SetEffect: { effect: { [effectName]: payload } } }
  }

  buttonConfigs.value[btnIndex].label = editData.value.label
  buttonConfigs.value[btnIndex].icon = editData.value.icon
  buttonConfigs.value[btnIndex].command = newCmd

  isConfigOpen.value = false
  await triggerAction(newCmd)
}

const triggerAction = async (command: any) => {
  try {
    await invoke('send_to_pico', { command })
    addLog(`CMD: ${typeof command === 'string' ? command : Object.keys(command)[0]}`)
  } catch (error) {
    addLog(`ERR: ${error}`)
  }
}

const addLog = (msg: string) => {
  picoLogs.value.unshift(msg)
  if (picoLogs.value.length > 8) picoLogs.value.pop()
}

onMounted(async () => {
  unlistenFn = await listen('pico-event', (event: any) => {
    const msg = event.payload
    if (msg.Log) {
      addLog(`SYS: ${msg.Log}`)
      if (!picoConnected.value) picoConnected.value = true
    }
  })
})

onUnmounted(() => {
  if (unlistenFn) unlistenFn()
})
</script>

<style scoped>
.app-background {
  background-color: #0A0A0C !important;
  color: #E0E0E0;
}

.tracking-widest {
  letter-spacing: 0.1em;
}
.letter-spacing-tight {
  letter-spacing: -0.05em;
}

.deck-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 24px;
  max-width: 700px;
  width: 100%;
}

.deck-key {
  aspect-ratio: 1;
  background: linear-gradient(145deg, #1A1A1F 0%, #121215 100%);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 20px;
  cursor: pointer;
  position: relative;
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 0 10px 30px -10px rgba(0,0,0,0.5), inset 0 1px 0 rgba(255,255,255,0.05);
}

.deck-key.hovered {
  transform: translateY(-4px);
  border-color: rgba(33, 150, 243, 0.3);
  box-shadow: 0 15px 35px -10px rgba(0,0,0,0.8), 0 0 20px -5px rgba(33, 150, 243, 0.2);
}

.deck-key.hovered:has(.text-error) {
  border-color: rgba(255, 82, 82, 0.4);
  box-shadow: 0 15px 35px -10px rgba(0,0,0,0.8), 0 0 20px -5px rgba(255, 82, 82, 0.2);
}

.deck-key.editing {
  border-color: #2196F3;
  box-shadow: 0 0 0 2px rgba(33, 150, 243, 0.2);
}

.key-content {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 12px;
}

.key-label {
  font-size: 0.75rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #9E9E9E;
  text-align: center;
  margin-top: 8px;
  transition: color 0.2s;
}

.deck-key.hovered .key-label {
  color: #FFFFFF;
}

.text-error {
  color: #FF5252 !important;
}

.drop-shadow {
  filter: drop-shadow(0 4px 6px rgba(0,0,0,0.5));
}

.border-subtle {
  border: 1px solid rgba(255, 255, 255, 0.05);
}
.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  box-shadow: 0 0 10px currentColor;
}

.log-window {
  background: #000000;
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 12px;
  padding: 12px 16px;
  height: 120px;
  overflow-y: auto;
  font-family: 'Fira Code', monospace;
  font-size: 0.8rem;
  box-shadow: inset 0 2px 10px rgba(0,0,0,0.5);
}

.log-entry {
  color: #A5D6A7;
  margin-bottom: 4px;
  line-height: 1.4;
}

.border-left {
  border-left: 1px solid rgba(255,255,255,0.05) !important;
}
</style>