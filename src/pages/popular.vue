<script setup lang="ts">
import Discover_package from '../components/discover_package.vue';

const barrells: Barrell[] = await (await fetch("https://api.fermentpkg.tech/barrells")).json()
const highestDownloaded: Barrell[] = barrells.sort((a, b) => b.downloads - a.downloads).slice(0, 3)

</script>

<template>
    <div class="local-container">
        <h1>Popular</h1>
        <div class="local-packages">
            <Discover_package v-for="barrell in highestDownloaded" :barrell="barrell" :key="barrell.name" />
        </div>
    </div>
</template>