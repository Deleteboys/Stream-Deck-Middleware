<template>
  <div class="pa-4">
    <div class="text-overline mb-4 text-grey">Bibliothek</div>
    <v-list bg-color="transparent" density="compact">
      <v-list-subheader class="text-primary font-weight-bold">STANDARD</v-list-subheader>
      <v-list-item v-for="a in actions" :key="a.title" :prepend-icon="a.icon" :title="a.title"
                   variant="tonal" class="mb-2 rounded-lg" link></v-list-item>
    </v-list>
  </div>
</template>

<script setup lang="ts">
const actions = [
  { title: 'App Starten', icon: 'mdi-rocket-launch' },
  { title: 'Lautstärke', icon: 'mdi-volume-high' },
  { title: 'OBS Szene', icon: 'mdi-video-switch' },
  { title: 'Webseite', icon: 'mdi-earth' }
]
</script>