<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { ref } from 'vue';
import Package from '../components/package.vue';
const props = defineProps<{ barrells: Barrell[] }>()
const showBarrellPage = ref<{ status: boolean, barrell?: Barrell }>({ status: false })
function listenToBarrellPage() {
    listen<string>("displayBarrellPage", ({ payload: barrell }) => {
        showBarrellPage.value.barrell = JSON.parse(barrell)
        showBarrellPage.value.status = true
        // unlisten()
    })


}
try {
    listenToBarrellPage()
}
catch (e) {
    console.error(e)
}
</script>

<template>
    <div class="local-container" v-if="!showBarrellPage.status">
        <h1>Search Result</h1>
        <h3>Barrells: {{barrells.length}}</h3>
        <div class="barrells" v-if="barrells.length > 0">

            <Package v-for="barrell in props.barrells" :barrell="barrell" :key="barrell.name" />
        </div>
        <h2 v-else>Nothing Found</h2>
    </div>
    <div class="local-container" v-else>
        <h1>{{showBarrellPage.barrell?.name}}</h1>
        <p>{{showBarrellPage.barrell?.description}}</p>
        <div v-if="showBarrellPage.barrell?.home">
            <span>Home: </span>
            <a :href="showBarrellPage.barrell?.home" target="_blank">{{showBarrellPage.barrell?.home}}</a>
        </div>
        <a target="_blank"
            :href="'https://fermentpkg.tech/barrells/'+showBarrellPage.barrell?.name">https://fermentpkg.tech/barrells/{{showBarrellPage.barrell?.name}}</a>
        <br>
        <div
            v-if="showBarrellPage.barrell?.dependencies||(showBarrellPage.barrell?.dependencies?.length as number) < 1">
            <h3>Dependencies</h3>
            <ul>
                <li v-for="dependency in showBarrellPage.barrell?.dependencies">{{dependency}}</li>
            </ul>


        </div>
        <button @click="()=>showBarrellPage.status=false">Return To Barrells Search</button>
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

a {
    display: inline-block;
    margin-bottom: 10px;
}

.local-container {}
</style>