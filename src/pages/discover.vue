<script setup lang="ts">

const barrells: Barrell[] = await(await fetch("https://api.fermentpkg.tech/barrells")).json()
</script>

<template>
    <div class="local-container">
        <h1>Discover</h1>
    </div>
</template>