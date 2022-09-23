import { createApp, defineAsyncComponent } from "vue";
import App from "./App.vue";
import "./style.css";
createApp(App)
.component("discover", defineAsyncComponent(() => import("./pages/discover.vue")))
.component("settings", defineAsyncComponent(() => import("./pages/settings.vue")))
.mount("#app");
