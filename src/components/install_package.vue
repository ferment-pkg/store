<script setup lang="ts">
import { ref } from 'vue';
const { barrell } = defineProps<{ barrell: Barrell }>()
let p = ref(0);
setInterval(() => {
    p.value = p.value + Math.floor(Math.random() * 10);
    if (p.value > 100) {
        p.value = 0;
    }
}, 1000);
</script>
<template>
    <div class="container">
        <p>{{barrell.name}}</p>
        <div class="progress">
            <div class="progress-bar" :style="{width: p + '%'}"></div>

        </div>
        <button @click="p=0">Cancel</button>


    </div>
</template>
<style scoped lang="scss">
.progress {
    width: 30%;
    height: 20px;
    background-color: #ddd;
    border-radius: 10px;
    z-index: 1;
    margin-right: 5px;



    .progress-bar {
        height: 100%;
        background-color: #1c9dd0;
        border-radius: 10px;
        transition: width 1s ease-in-out;
        //move 5 px up
        transform: translateY(-10px);

    }
}

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