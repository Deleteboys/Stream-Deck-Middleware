<template>
  <div class="pa-6">
    <div v-if="store.selectedElementId" class="mb-4">
      <v-btn
          prepend-icon="mdi-chevron-left"
          variant="text"
          @click="store.selectElement(null)"
          class="text-none"
      >
        Zurück zu LEDs
      </v-btn>
    </div>

    <v-fade-transition mode="out-in">

      <LedConfigSection v-if="!store.selectedElementId" />

      <div v-else>
        <h3 class="text-h6 font-weight-bold mb-6">
          <v-icon color="primary" class="mr-2">mdi-tune</v-icon>
          {{ store.selectedElementId.includes('btn') ? 'Button' : 'Encoder' }} Settings
        </h3>
        <v-text-field label="Label" variant="outlined" density="comfortable"></v-text-field>
        <v-select :items="['Hotkey', 'Media']" label="Aktion" variant="outlined"></v-select>
        <v-btn color="primary" block class="mt-4">Speichern</v-btn>
      </div>

    </v-fade-transition>
  </div>
</template>

<script setup lang="ts">
import { useStreamDeckStore } from '@/stores/streamdeck';
import LedConfigSection from './LedConfigSection.vue'; // Die neue Datei importieren

const store = useStreamDeckStore();
</script>