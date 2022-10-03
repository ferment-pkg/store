<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import { ref } from 'vue';
const { barrell } = defineProps<{ barrell: Barrell }>()
function displayBarrellPage(event: MouseEvent) {
    event.preventDefault()
    emit('displayBarrellPage', barrell)
}
let p = ref(0);

</script>
<template>
    <div class="container">
        <p>{{barrell.name}}</p>

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