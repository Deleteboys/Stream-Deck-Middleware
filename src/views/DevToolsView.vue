<template>
  <div class="h-100 w-100 d-flex flex-column pa-6 bg-zinc-950 overflow-hidden custom-scrollbar">

    <!-- HEADER -->
    <div class="flex-grow-0 mb-6">
      <h2 class="text-h4 font-weight-bold text-white mb-1">Developer Console</h2>
      <p class="text-grey">System-Konsolenausgabe und Debug-Logs.</p>
    </div>

    <!-- TERMINAL / LOG VIEWER KACHEL -->
    <v-card
        class="flex-grow-1 d-flex flex-column overflow-hidden"
        elevation="0"
        rounded="xl"
        color="#18181b"
        border
        style="border-color: rgba(255, 255, 255, 0.05) !important;"
    >
      <!-- Terminal Header mit Filtern -->
      <div class="d-flex align-center px-4 py-2 border-b flex-wrap"
           style="border-color: rgba(255, 255, 255, 0.05) !important; background-color: rgba(0,0,0,0.2);">

        <v-icon icon="mdi-console" color="grey" size="small" class="mr-3"></v-icon>
        <div class="text-caption text-grey text-uppercase font-weight-bold letter-spacing-1 mr-4">
          Live Logs
        </div>

        <v-divider vertical class="mx-2 my-2" color="grey-darken-3"></v-divider>

        <!-- Log Level Toggles -->
        <v-chip-group
            v-model="activeLogLevels"
            multiple
            selected-class="text-white"
            class="ma-0 ml-2"
        >
          <v-chip
              v-for="level in logLevels"
              :key="level.value"
              :value="level.value"
              :color="level.color"
              filter
              variant="tonal"
              size="small"
              class="font-weight-bold text-caption"
          >
            {{ level.name }}
          </v-chip>
        </v-chip-group>

        <v-spacer></v-spacer>

        <v-btn
            size="small"
            variant="text"
            color="grey-lighten-1"
            prepend-icon="mdi-delete"
            class="text-none"
            @click="store.clearLogs()"
        >
          Leeren
        </v-btn>
      </div>

      <!-- Terminal Output -->
      <div class="flex-grow-1 overflow-y-auto pa-4 terminal-window custom-scrollbar" style="background-color: #09090b;">
        <div v-if="filteredLogs.length === 0"
             class="h-100 d-flex flex-column align-center justify-center text-grey-darken-2">
          <v-icon icon="mdi-sleep" size="x-large" class="mb-3 opacity-50"></v-icon>
          <div>
            {{
              store.debugLogs.length === 0 ? 'Warte auf Systemereignisse...' : 'Keine Logs für gewählte Filter gefunden.'
            }}
          </div>
        </div>

        <div
            v-for="(log, idx) in filteredLogs"
            :key="idx"
            class="log-entry d-flex py-1"
        >
          <span class="log-time text-grey-darken-1 mr-4">[{{ log.timestamp }}]</span>
          <span :class="getLogLevelClass(log.level)" class="log-level mr-4 font-weight-bold">
            [{{ getLogLevelName(log.level) }}]
          </span>
          <span class="log-message text-grey-lighten-1">{{ log.message }}</span>
        </div>
      </div>
    </v-card>

  </div>
</template>

<script setup lang="ts">
import {ref, computed} from "vue";
import {useStreamDeckStore} from "@/stores/streamdeck";

const store = useStreamDeckStore();

// --- Log Filter Logik ---

// Definition der verfügbaren Log-Level mit ihren UI-Eigenschaften
const logLevels = [
  {value: 1, name: 'ERROR', color: 'error'},
  {value: 2, name: 'WARN', color: 'warning'},
  {value: 3, name: 'INFO', color: 'primary'},
  {value: 4, name: 'DEBUG', color: 'grey'},
  {value: 5, name: 'TRACE', color: 'grey-darken-2'},
];

const activeLogLevels = ref<number[]>([1, 2, 3]);

const filteredLogs = computed(() => {
  return store.debugLogs.filter(log => activeLogLevels.value.includes(log.level));
});

// --- Log Formatierung ---
const getLogLevelName = (level: number) => {
  switch (level) {
    case 1:
      return 'ERROR';
    case 2:
      return 'WARN';
    case 3:
      return 'INFO';
    case 4:
      return 'DEBUG';
    case 5:
      return 'TRACE';
    default:
      return 'LOG';
  }
};

const getLogLevelClass = (level: number) => {
  switch (level) {
    case 1:
      return 'text-error';
    case 2:
      return 'text-warning';
    case 3:
      return 'text-primary';
    case 4:
      return 'text-grey';
    case 5:
      return 'text-grey-darken-2';
    default:
      return 'text-white';
  }
};
</script>

<style scoped>
.bg-zinc-950 {
  background-color: #09090b !important;
}

.terminal-window {
  font-family: 'Fira Code', 'Courier New', Courier, monospace;
  font-size: 13px;
  line-height: 1.6;
}

.log-entry {
  word-break: break-all;
  border-bottom: 1px solid rgba(255, 255, 255, 0.02);
}

.log-entry:hover {
  background-color: rgba(255, 255, 255, 0.03);
}

.log-time {
  user-select: none;
  min-width: 85px;
}

.log-level {
  user-select: none;
  min-width: 70px;
  white-space: pre; /* Verhindert, dass das Leerzeichen bei WARN / INFO verschluckt wird */
}

.letter-spacing-1 {
  letter-spacing: 1px;
}

/* Custom Scrollbar */
.custom-scrollbar::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.custom-scrollbar::-webkit-scrollbar-track {
  background: transparent;
}

.custom-scrollbar::-webkit-scrollbar-thumb {
  background: #27272a;
  border-radius: 4px;
}

.custom-scrollbar::-webkit-scrollbar-thumb:hover {
  background: #3f3f46;
}
</style>