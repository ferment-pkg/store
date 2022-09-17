<script setup lang="ts">
defineProps<{ barrell: Barrell }>()
</script>
<template>
    <div class="container">
        <a :href="'https://fermentpkg.tech/barrells/'+barrell.name" target="_blank">{{barrell.name}}</a>
        {{barrell.description}}
    </div>
</template>