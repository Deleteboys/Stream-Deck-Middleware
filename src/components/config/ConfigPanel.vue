<template>
  <div class="pa-6">
    <div v-if="!store.selectedElementId" class="text-center mt-12 text-grey">
      <v-icon size="48" class="mb-4">mdi-cursor-default-click</v-icon>
      <p>Wähle einen Button aus</p>
    </div>

    <div v-else>
      <h3 class="mb-6 d-flex align-center">
        <v-icon color="primary" class="mr-2">mdi-tune</v-icon> Settings
      </h3>
      <v-text-field label="Label" variant="outlined" density="comfortable" class="mb-2"></v-text-field>
      <v-select :items="['Hotkey', 'Media', 'Macro']" label="Aktion" variant="outlined" density="comfortable"></v-select>
      <v-btn color="primary" block class="mt-4" rounded="lg">Speichern</v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useStreamDeckStore } from '@/stores/streamdeck'
const store = useStreamDeckStore()
</script>