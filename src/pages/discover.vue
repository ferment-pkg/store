<script setup lang="ts">
import Discover_package from '../components/discover_package.vue';

const barrells: Barrell[] = await (await fetch("https://api.fermentpkg.tech/barrells")).json()
const randomBarrells: Barrell[] = barrells.sort(() => Math.random() - 0.5).slice(0, 5)
</script>

<template>
    <div class="local-container">
        <h1>Discover</h1>
        <div class="local-packages">
            <Discover_package v-for="barrell in randomBarrells" :barrell="barrell" :key="barrell.name" />
        </div>
    </div>
</template>