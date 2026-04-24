<template>
  <div class="oled-wrapper">
    <div class="oled-container">
      <div class="oled-header">
        {{ store.activeProfile?.name || 'PROFIL: MAIN' }}
      </div>

      <div class="oled-divider-h"></div>

      <div class="oled-content">
        <div
            v-for="(segment, i) in segments"
            :key="i"
            class="oled-segment"
        >
          <div v-if="i > 0" class="oled-divider-v"></div>

          <div class="segment-inner">
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
              {{ store.activeProfile?.keys[`enc-${i}`]?.value ?? VOLUMES[i] }}%
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useStreamDeckStore } from '@/stores/streamdeck';

const store = useStreamDeckStore();

// Fallback Daten wie im Rust Code
const VOLUMES = [50, 65, 80, 35];

const ICONS = {
  MASTER: [
    "              ", "       11     ", "      111     ", "     1111   1 ",
    "   111111   11", "   111111  111", "   111111  111", "   111111  111",
    "   111111  111", "   111111   11", "     1111   1 ", "      111     ",
    "       11     ", "              "
  ],
  SPOTIFY: [
    "            ", "    111111   ", "  1111111111 ", " 111111111111",
    " 11        11", "11111111111111", "1111      1111", "11111111111111",
    " 1111    1111", " 111111111111", "  1111111111 ", "    111111   ",
    "            ", "            "
  ],
  DISCORD: [
    "            ", "            ", "            ", "   11  11   ",
    "  11111111  ", " 1111111111 ", " 11 1111 11 ", " 11 1111 11 ",
    " 1111111111 ", "  111  111  ", "   11  11   ", "            ",
    "            ", "            "
  ],
  BROWSER: [
    "              ", "     11111    ", "   111   111  ", "  111     111 ",
    " 111       111", " 111   1111111", " 111   1111111", " 111          ",
    " 111          ", "  111     11  ", "   111111111  ", "     11111    ",
    "              ", "              "
  ]
};

const segments = computed(() => {
  const iconKeys = ['MASTER', 'SPOTIFY', 'DISCORD', 'BROWSER'] as const;
  return iconKeys.map(key => ({
    iconMatrix: ICONS[key].map(row => row.split(''))
  }));
});
</script>

<style scoped>
/* Wrapper jetzt ohne Hintergrund und Padding */
.oled-wrapper {
  display: inline-block;
}

.oled-container {
  width: 256px; /* 128px * 2 */
  height: 128px; /* 64px * 2 */
  background-color: #000;
  color: #fff;
  font-family: 'Courier New', Courier, monospace;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  border: 4px solid #333;
  image-rendering: pixelated;
}

/* ... Rest des CSS bleibt gleich ... */

.oled-header {
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  text-transform: uppercase;
}

.oled-divider-h {
  height: 0;
  border-top: 2px dashed #555;
}

.oled-content {
  flex: 1;
  display: flex;
}

.oled-segment {
  flex: 1;
  display: flex;
  position: relative;
}

.oled-divider-v {
  width: 0;
  height: 80%;
  align-self: center;
  border-left: 2px dashed #555;
}

.segment-inner {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-evenly;
  padding: 5px 0;
}

.icon-grid {
  display: flex;
  flex-direction: column;
}

.icon-row {
  display: flex;
}

.pixel {
  width: 2px;
  height: 2px;
  background-color: transparent;
}

.pixel.on {
  background-color: #fff;
  box-shadow: 0 0 1px #fff;
}

.volume-text {
  font-size: 16px;
  font-weight: bold;
}
</style>