<script setup lang="ts">

const { barrell, progress } = defineProps<{
    barrell: Barrell, progress: "downloading" | "extracting" | "installingDependencies" | "installing" | "testing" | "done"
}>()
const progressDict = {
    "unknown": 0,
    "downloading": 10,
    "extracting": 20,
    "installingDependencies": 35,
    "installing": 50,
    "testing": 80,
    "done": 100
}
const readingDict = {
    "unknown": "Unknown",
    "downloading": "Downloading",
    "extracting": "Extracting",
    "installingDependencies": `Installing Dependencies`,
    "installing": "Installing",
    "testing": "Testing",
    "done": "Done"
}
</script>
<template>
    <div class="container">
        <p>{{barrell.name}}</p>
        <div class="progress">
            <div class="progress-bar" :style="{width:  progressDict[progress || 'unknown']+ '%' }"></div>

        </div>
        <span>{{progressDict[progress || "unknown"]}}%</span>
        <strong>
            STATUS: {{readingDict[progress || "unknown"]}}
        </strong>
        <button @click="">Cancel</button>


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