<template>
  <v-container fluid class="pa-0 fill-height bg-neutral-950">
    <v-row no-gutters class="fill-height">
      <v-col cols="12" md="3" lg="2" class="bg-zinc-900 border-right">
        <ActionSidebar />
      </v-col>

      <v-col cols="12" md="6" lg="8" class="d-flex align-center justify-center bg-zinc-950 main-viewer">
        <HardwareBoard />
      </v-col>

      <v-col cols="12" md="3" lg="2" class="bg-zinc-900 border-left">
        <ConfigPanel />
      </v-col>
    </v-row>
  </v-container>
</template>

<style scoped>
/* Farben für ein stimmiges Gesamtbild */
.bg-zinc-950 { background-color: #09090b !important; } /* Tiefstes Dunkelgrau */
.bg-zinc-900 { background-color: #121214 !important; } /* Panel-Farbe */

.border-right { border-right: 1px solid rgba(255, 255, 255, 0.05) !important; }
.border-left { border-left: 1px solid rgba(255, 255, 255, 0.05) !important; }

/* Subtiler Glow-Effekt hinter dem Deck */
.main-viewer {
  background: radial-gradient(circle at center, #18181b 0%, #09090b 100%);
}
</style>

<script setup lang="ts">
import ActionSidebar from '@/components/config/ActionSidebar.vue'
import HardwareBoard from '@/components/config/HardwareBoard.vue'
import ConfigPanel from '@/components/config/ConfigPanel.vue'
</script>
