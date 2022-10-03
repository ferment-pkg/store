import { createApp, defineAsyncComponent } from "vue";
import App from "./App.vue";
import "./style.css";
createApp(App)
.component("discover", defineAsyncComponent(() => import("./pages/discover.vue")))
.component("settings", defineAsyncComponent(() => import("./pages/settings.vue")))
.component("popular", defineAsyncComponent(() => import("./pages/popular.vue")))
.component("install", defineAsyncComponent(() => import("./pages/installs.vue")))
.mount("#app");
