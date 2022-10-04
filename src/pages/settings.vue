<script setup lang="ts">
import { SettingsManager } from 'tauri-settings';
import { ref } from 'vue';
const settings = ref<Settings>({
    packageManagerPath: "/usr/local/ferment/ferment",
    apiPath: "https://api.fermentpkg.tech/",
    installOptions: [],
});
const manager = new SettingsManager<Settings>({
    ...settings.value
}, {
    fileName: "settings",
})
await manager.initialize();
for (const key in settings.value) {
    // @ts-ignore
    settings.value[key] = manager.getCache(key as keyof Settings) || await manager.get(key as keyof Settings);
}
async function saveSettings() {
    for (const key in settings.value) {
        // @ts-ignore
        manager.setCache(key, settings.value[key]);
    }
    await manager.syncCache()
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