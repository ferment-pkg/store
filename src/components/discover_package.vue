<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
const { barrell } = defineProps<{ barrell: Barrell }>()
function displayBarrellPage(event: MouseEvent) {
    event.preventDefault()
    emit('displayBarrellPage', barrell)
}
</script>
<template>
    <div class="container">
        <a @click="(event)=>displayBarrellPage(event)">{{barrell.name}}</a>
        {{barrell.description}}
        <p><strong>Downloads: {{Intl.NumberFormat("en",
        {notation:"compact"}).format(barrell.downloads)}}</strong></p>
        <button class="install">Install</button>
    </div>
</template>
<style scoped lang="scss">
.container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;


    * {
        margin-top: 10px;
    }

    a {
        cursor: pointer;
    }
}

.hidden {
    display: none;
}

button.install {
    width: 20%;

    &:hover {
        background-color: rgb(172, 176, 181);
        color: black;
    }

}
</style>