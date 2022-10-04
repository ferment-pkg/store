<script setup lang="ts">
import Install_package from '../components/install_package.vue';

const barrells: Barrell[] = await (await fetch("https://api.fermentpkg.tech/barrells")).json()
const highestDownloaded: Barrell[] = barrells.sort((a, b) => b.downloads - a.downloads).slice(0, 3)

</script>

<template>
    <div class="local-container">
        <h1>Installs</h1>
        <div class="local-packages">
            <Install_package v-for="barrell in highestDownloaded" :barrell="barrell" :key="barrell.name"
                progress="testing" />
        </div>
    </div>
</template>