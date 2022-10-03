import { createApp, defineAsyncComponent } from "vue";
import CircleProgress from 'vue3-circle-progress';
import App from "./App.vue";
import "./style.css";
createApp(App)
.component("discover", defineAsyncComponent(() => import("./pages/discover.vue")))
.component("settings", defineAsyncComponent(() => import("./pages/settings.vue")))
.component("popular", defineAsyncComponent(() => import("./pages/popular.vue")))
.component("install", defineAsyncComponent(() => import("./pages/installs.vue")))
.use(CircleProgress)
.mount("#app");
