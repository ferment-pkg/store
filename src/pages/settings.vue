<script setup lang="ts">
import { ref } from 'vue';
const settings = ref<Settings>({
    packageManagerPath: "/usr/local/ferment/ferment",
    apiPath: "https://api.fermentpkg.tech/",
    installOptions: [],
});
settings.value = localStorage.getItem("settings") ? JSON.parse(localStorage.getItem("settings") as string) : settings.value;
function saveSettings() {
    localStorage.setItem("settings", JSON.stringify(settings.value));
}
function resetSettings() {
    settings.value = {
        packageManagerPath: "/usr/local/ferment/ferment",
        apiPath: "https://api.fermentpkg.tech/",
        installOptions: [],
    };
    saveSettings();
}
</script>
<template>
    <div class="local-container">
        <h1>Settings</h1>
        <span>Package Manager Path: </span>
        <input type="text" v-model="settings.packageManagerPath" @change="saveSettings()"
            placeholder="/usr/local/ferment/ferment" />
        <span>API Path: </span>
        <input type="text" v-model="settings.apiPath" @change="saveSettings()"
            placeholder="https://api.fermentpkg.tech/" />
        <button @click="resetSettings()">Default</button>
    </div>
</template>
<style scoped lang="scss">
.local-container {
    display: flex;
    flex-direction: column;
    align-items: center;

    input {
        margin-bottom: 1rem;
        width: 45%;
        text-align: center;

    }

    button {
        background-color: rgb(159, 34, 34);
    }

}
</style>