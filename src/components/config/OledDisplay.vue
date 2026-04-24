<template>
  <div class="oled-container">
    <div class="oled-header">
      {{ store.activeProfile?.name || 'NO PROFILE' }}
    </div>

    <div class="oled-divider-h"></div>

    <div class="oled-content">
      <div
          v-for="(segment, i) in segments"
          :key="i"
          class="oled-segment"
          :class="{ 'with-divider': i > 0 }"
      >
        <div class="icon-grid">
          <div v-for="(row, ri) in segment.iconMatrix" :key="ri" class="icon-row">
            <div
                v-for="(pixel, pi) in row"
                :key="pi"
                :class="['pixel', { on: pixel === '1' }]"
            ></div>
          </div>
        </div>

        <div class="volume-text">
          {{ store.activeProfile?.keys[`enc-${i}`]?.value ?? 50 }}%
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useStreamDeckStore } from '@/stores/streamdeck';

const store = useStreamDeckStore();

// Die Icons aus deinem Rust-Code konvertiert
const ICONS = {
  MASTER: ["00000000000000","00000001100000","00000011100000","00000111100010","00011111100011","00011111100111","00011111100111","00011111100111","00011111100111","00011111100011","00000111100010","00000011100000","00000001100000","00000000000000"],
  SPOTIFY: ["000000000000","000011111100","001111111111","011111111111","011000000011","111111111111","111100001111","111111111111","011110001110","011111111111","001111111111","000011111100","000000000000","000000000000"],
  DISCORD: ["000000000000","000000000000","000000000000","000110011000","001111111100","011111111110","011011110110","011011110110","011111111110","001110011100","000110011000","000000000000","000000000000","000000000000"],
  BROWSER: ["00000000000000","00000111110000","00011100011100","00111000001110","01110000000111","01110001111111","01110001111111","01110000000000","01110000000000","00111000001100","00011111111100","00000111110000","00000000000000","00000000000000"]
};

const segments = computed(() => {
  const iconKeys = ['MASTER', 'SPOTIFY', 'DISCORD', 'BROWSER'];
  return iconKeys.map(key => ({
    iconMatrix: ICONS[key as keyof typeof ICONS].map(row => row.split(''))
  }));
});
</script>

<style scoped>
.oled-container {
  width: 256px; /* 128px * 2 für bessere Sichtbarkeit */
  height: 128px; /* 64px * 2 */
  background-color: #000;
  color: #fff;
  font-family: 'Courier New', Courier, monospace;
  display: flex;
  flex-direction: column;
  border: 4px solid #333;
  image-rendering: pixelated;
  user-select: none;
}

.oled-header {
  height: 32px; /* Entspricht den ersten 16px im Rust Code */
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  font-weight: bold;
  letter-spacing: 1px;
}

.oled-divider-h {
  height: 2px;
  border-top: 2px dashed #444;
}

.oled-content {
  flex: 1;
  display: flex;
}

.oled-segment {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-around;
  padding: 4px 0;
}

.oled-segment.with-divider {
  border-left: 2px dashed #444;
}

/* Icon Pixel Rendering */
.icon-grid {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.icon-row {
  display: flex;
  gap: 0;
}

.pixel {
  width: 2px;
  height: 2px;
  background-color: transparent;
}

.pixel.on {
  background-color: #fff;
  box-shadow: 0 0 2px rgba(255, 255, 255, 0.5);
}

.volume-text {
  font-size: 16px;
  font-weight: bold;
}
</style>