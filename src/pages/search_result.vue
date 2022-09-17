<script setup lang="ts">
import Package from '../components/package.vue';
const props = defineProps<{ barrells: Barrell[] }>()
</script>

<template>
    <div class="local-container">
        <h1>Search Result</h1>
        <h3>Barrells: {{barrells.length}}</h3>
        <div class="barrells" v-if="barrells.length > 0">

            <Package v-for="barrell in props.barrells" :barrell="barrell" :key="barrell.name" />
        </div>
        <h2 v-else>Nothing Found</h2>
    </div>
</template>
<style scoped lang="scss">
$color: rgb(25, 26, 27);

h1 {
    margin-top: 40px;
}

h2 {
    position: absolute;
    top: 50%;
    left: 60%;
    transform: translate(-50%, -50%);
}
</style>