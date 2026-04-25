<template>
  <v-container class="py-10">
    <v-card max-width="500" class="mx-auto pa-6" border variant="flat" color="#1a1a1c">

      <h2 class="text-subtitle-1 text-medium-emphasis text-uppercase font-weight-bold letter-spacing-1 border-b pb-3 mb-6">
        Firmware
      </h2>

      <v-window v-model="updatePhase" :touch="false">

        <v-window-item :value="1">
          <div class="text-center mb-6">
            <v-icon size="48" color="primary" class="mb-3">mdi-tray-arrow-down</v-icon>
            <h3 class="text-h5 font-weight-bold mb-1">Update verfügbar</h3>
            <p class="text-body-2 text-medium-emphasis">Eine neue Firmware-Version steht bereit.</p>
          </div>

          <v-card variant="outlined" class="d-flex align-center justify-center pa-5 mb-8 border-dashed bg-transparent">
            <div class="text-center flex-1-1-0">
              <div class="text-caption text-uppercase text-medium-emphasis mb-1">Aktuell</div>
              <div class="text-h5 font-weight-bold font-monospace">{{ currentVersion }}</div>
            </div>

            <v-icon color="grey-darken-1" size="x-large" class="mx-2 opacity-50">mdi-arrow-right</v-icon>

            <div class="text-center flex-1-1-0">
              <div class="text-caption text-uppercase text-medium-emphasis mb-1">Neu</div>
              <div class="text-h5 font-weight-bold text-primary font-monospace">{{ newVersion }}</div>
            </div>
          </v-card>

          <v-btn
              color="primary"
              block
              size="x-large"
              class="font-weight-bold text-none"
              elevation="4"
              @click="startUpdate"
          >
            Jetzt aktualisieren
          </v-btn>
        </v-window-item>

        <v-window-item :value="2">
          <div class="text-center py-4">
            <v-progress-circular
                indeterminate
                color="primary"
                size="56"
                width="4"
                class="mb-6"
            ></v-progress-circular>

            <h3 class="text-h6 font-weight-medium mb-6">{{ statusMessage }}</h3>

            <v-progress-linear
                v-model="updateProgress"
                color="primary"
                height="8"
                rounded
                class="mb-8"
            ></v-progress-linear>

            <v-alert
                type="error"
                variant="tonal"
                class="text-left text-body-2 font-weight-medium"
                icon="mdi-power-plug-off"
            >
              Bitte trenne das DIY Deck nicht vom Strom oder PC!
            </v-alert>
          </div>
        </v-window-item>

        <v-window-item :value="3">
          <div class="text-center py-6">
            <v-icon color="success" size="72" class="mb-4">mdi-check-circle</v-icon>
            <h3 class="text-h5 font-weight-bold text-success mb-2">Update erfolgreich!</h3>
            <p class="text-body-2 text-medium-emphasis mb-8">Dein Gerät wurde aktualisiert und startet neu.</p>
            <v-btn variant="tonal" block size="large" @click="resetUpdate">Zurück zu den Einstellungen</v-btn>
          </div>
        </v-window-item>

      </v-window>

      <v-divider class="my-8 opacity-20"></v-divider>

      <h2 class="text-subtitle-1 text-medium-emphasis text-uppercase font-weight-bold letter-spacing-1 border-b pb-3 mb-6">
        System-Einstellungen
      </h2>
      <v-switch label="Auto-Start mit Windows" color="primary" hide-details inset></v-switch>
      <v-switch label="Vibration standardmäßig an" color="primary" hide-details inset></v-switch>

    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { startBootloader } from "@/services/streamdeckCommands";

// State für das Update-UI
const updatePhase = ref(1); // 1 = Info, 2 = Laden, 3 = Erfolgreich
const updateProgress = ref(0);
const statusMessage = ref("");

// Mock-Daten für die Versionen
const currentVersion = ref("v1.0.4");
const newVersion = ref("v1.1.0");

const startUpdate = async () => {
  // UI auf Ladebildschirm umstellen
  updatePhase.value = 2;
  updateProgress.value = 0;
  statusMessage.value = "Starte Bootloader...";

  try {
    // 1. Bootloader des Geräts triggern
    await startBootloader();
    statusMessage.value = "Flashe Firmware... (0%)";

    // 2. Simulation des Flash-Vorgangs (Fortschrittsbalken)
    const interval = setInterval(() => {
      updateProgress.value += Math.random() * 4;

      if (updateProgress.value > 30 && updateProgress.value < 70) {
        statusMessage.value = `Flashe Firmware... (${Math.round(updateProgress.value)}%)`;
      } else if (updateProgress.value >= 70) {
        statusMessage.value = "Überprüfe Installation...";
      }

      // 3. Update abgeschlossen
      if (updateProgress.value >= 100) {
        updateProgress.value = 100;
        clearInterval(interval);

        setTimeout(() => {
          updatePhase.value = 3;
          currentVersion.value = newVersion.value; // Lokale Version visuell aktualisieren
        }, 800);
      }
    }, 150);

  } catch (error) {
    // Falls der Bootloader-Befehl fehlschlägt
    updatePhase.value = 1;
    alert(`Firmware-Update konnte nicht gestartet werden: ${String(error)}`);
  }
};

const resetUpdate = () => {
  updatePhase.value = 1;
  updateProgress.value = 0;
};
</script>

<style scoped>
/* Hilfsklassen für sauberes Typography- & Border-Styling */
.letter-spacing-1 {
  letter-spacing: 1px;
}

.font-monospace {
  font-family: 'Courier New', Courier, monospace;
}

.border-b {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08) !important;
}

.border-dashed {
  border-style: dashed !important;
  border-width: 1px;
  border-color: rgba(255, 255, 255, 0.15) !important;
}

.opacity-50 {
  opacity: 0.5;
}

.opacity-20 {
  opacity: 0.2;
}

/* Verhindert, dass das V-Window seine Höhe wild animiert */
:deep(.v-window) {
  min-height: 280px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>