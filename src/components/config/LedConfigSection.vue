<template>
  <div class="led-config">
    <h3 class="mb-6 d-flex align-center text-uppercase text-caption font-weight-bold text-grey">
      <v-icon size="small" class="mr-2">mdi-led-variant</v-icon> Global LED Effects
    </h3>

    <v-select
        v-model="store.ledConfig.effect"
        :items="Object.keys(EFFECT_PARAMS)"
        label="Effekt"
        variant="filled"
        density="compact"
        rounded="lg"
        class="mb-4"
    ></v-select>

    <v-divider class="mb-6"></v-divider>

    <v-window v-model="store.ledConfig.effect">
      <div v-for="(params, effectName) in EFFECT_PARAMS" :key="effectName">
        <div v-if="store.ledConfig.effect === effectName" class="animate-fade-in">

          <div v-for="param in params" :key="param" class="control-group mb-4">

            <template v-if="param === 'color'">
              <label class="text-caption text-grey mb-2 d-block text-uppercase font-weight-medium">Farbe</label>
              <v-color-picker
                  v-model="store.ledConfig.color"
                  hide-inputs flat width="100%" canvas-height="80"
                  class="bg-transparent border rounded-lg"
              ></v-color-picker>
            </template>

            <template v-else>
              <div class="d-flex justify-space-between align-center mb-1">
                <label class="text-caption text-grey text-uppercase">{{ param.replace('_', ' ') }}</label>
                <span class="text-caption font-weight-bold text-primary">{{ Math.round(store.ledConfig[param]) }}</span>
              </div>
              <v-slider
                  v-model="store.ledConfig[param]"
                  :min="0"
                  :max="param === 'hue' ? 360 : 255"
                  step="1"
                  hide-details
                  color="primary"
                  density="compact"
                  @update:model-value="(val) => store.ledConfig[param] = Math.round(val)"
              ></v-slider>
            </template>

          </div>
        </div>
      </div>
    </v-window>
  </div>
</template>

<script setup lang="ts">
import { useStreamDeckStore } from '@/stores/streamdeck';

const store = useStreamDeckStore();

const EFFECT_PARAMS = {
  Solid: ['color', 'brightness'],
  Blink: ['color', 'brightness', 'speed'],
  Rainbow: ['brightness', 'speed'],
  Breathing: ['color', 'brightness', 'speed'],
  Chase: ['color', 'brightness', 'speed', 'size'],
  Comet: ['color', 'brightness', 'speed', 'tail'],
  Sparkle: ['color', 'brightness', 'speed', 'density'],
  ColorOrbit: ['hue', 'hue_shift', 'saturation', 'brightness', 'speed'],
  Astolfo: ['brightness', 'speed', 'saturation', 'spread']
};
</script>

<style scoped>
.control-group {
  background: rgba(255, 255, 255, 0.03);
  padding: 12px;
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.05);
}
.text-primary {
  color: #6366f1 !important;
}
.animate-fade-in {
  animation: fadeIn 0.3s ease-in-out;
}
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(5px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>