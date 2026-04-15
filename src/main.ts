import { createApp } from "vue";
import ui from "@nuxt/ui/vue-plugin";
import App from "./App.vue";
import { useProfilesStore } from "./stores/profiles";
import "./assets/main.css";

async function bootstrap() {
  const app = createApp(App);
  app.use(ui);
  try {
    await useProfilesStore().hydrate();
  } catch (err) {
    console.error("failed to hydrate profiles store", err);
  }
  app.mount("#app");
}

void bootstrap();
