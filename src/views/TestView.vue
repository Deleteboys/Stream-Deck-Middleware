<template>
  <v-container class="py-10">
    <v-row>
      <v-col cols="12">
        <h2 class="text-h4 mb-2">Hardware Test</h2>
        <p class="text-grey">Teste die LEDs und Funktionen deines Decks.</p>
      </v-col>
      <v-col v-for="n in 12" :key="n" cols="6" sm="4" md="2">
        <v-btn block height="80" variant="outlined" color="primary" @click="testLed(n)">
          LED {{ n }}
        </v-btn>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
const testLed = (id: number) => console.log("Test LED:", id)
</script>