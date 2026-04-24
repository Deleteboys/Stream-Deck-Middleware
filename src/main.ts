import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import vuetify from './plugins/vuetify';
import router, { getRestorableRoute } from './router';

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.use(router);
app.use(vuetify);

const mountApp = async () => {
  await router.isReady();

  const lastRoute = getRestorableRoute();
  if (lastRoute && router.currentRoute.value.path !== lastRoute) {
    await router.replace(lastRoute).catch(() => undefined);
  }

  app.mount('#app');
};

void mountApp();
