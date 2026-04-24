import { createApp } from "vue";
import { createPinia } from "pinia"; // 1. Pinia importieren
import App from "./App.vue";
import vuetify from './plugins/vuetify';
import router from './router'; // 2. Deinen Router importieren

const app = createApp(App);
const pinia = createPinia(); // 3. Pinia Instanz erstellen

app.use(pinia);   // Zuerst den Store
app.use(router);  // Dann den Router
app.use(vuetify); // Dann das UI-Framework

app.mount('#app');