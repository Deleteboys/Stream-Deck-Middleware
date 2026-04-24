<template>
  <v-container class="py-10">
    <v-card max-width="600" class="mx-auto pa-6" border variant="flat">
      <h2 class="mb-6">System-Einstellungen</h2>
      <v-switch label="Auto-Start mit Windows" color="primary" hide-details></v-switch>
      <v-switch label="Vibration standardmäßig an" color="primary" hide-details></v-switch>
      <v-divider class="my-6"></v-divider>
      <v-btn color="error" variant="outlined">Firmware Update erzwingen</v-btn>
    </v-card>
  </v-container>
</template>