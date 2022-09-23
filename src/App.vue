<script setup lang="ts">
import { emit, listen } from '@tauri-apps/api/event';
import { ref } from 'vue';
import Search from './pages/search_result.vue';
const page = ref<string>("discover")
const isSearch = ref<boolean>(false)
const barrells = ref<Barrell[]>([])
const search = ref<string>("")

async function getBarrells() {
  barrells.value = await (await fetch("https://api.fermentpkg.tech/barrells", { cache: "default" })).json()
}
function setPage(pageName: string) {
  emit("pageChange", { oldPage: page.value })
  page.value = pageName
  isSearch.value = false

}

getBarrells()
const filteredBarrells = ref<Barrell[]>(barrells.value)
function searchForPackage() {

  isSearch.value = true
  page.value = ""
  filteredBarrells.value = barrells.value.filter(barrell => barrell.name.toLowerCase().includes(search.value.toLowerCase()))
  emit("pageChange", { oldPage: page.value })
}
listen<string>("changePage", (page) => {
  setPage(page.payload)
  isSearch.value = false
})
</script>

<template>
  <div class="container">
    <nav>
      <a :class="page=='discover'?'selected':null" @click="setPage('discover')">Discover</a>
      <a :class="page=='popular'?'selected':null" @click="setPage('popular')">Popular</a>
      <a :class="page=='install'?'selected':null" @click="setPage('install')">Installs</a>
      <a :class="page=='settings'?'selected':null" @click="setPage('settings')">Settings</a>
      <div class="search">
        <input type="text" v-model="search" placeholder="Search for a package">
        <button @click="searchForPackage()">Search</button>
      </div>
    </nav>
    <div class="content">
      <Transition name="slide-fade">
        <Suspense v-if="!isSearch">
          <component :is="page"></component>

        </Suspense>
      </Transition>
      <Transition name="slide-fade">
        <Suspense v-if="isSearch">
          <Search :barrells="filteredBarrells" />
        </Suspense>
      </Transition>

    </div>
  </div>
</template>

<style scoped lang="scss">
@import "./styles/main.scss";

.slide-fade-enter-active {
  transition: all 0.3s ease-out;
  transition-delay: 0.9s;
}

.slide-fade-leave-active {
  transition: all 0.7s cubic-bezier(1, 0.2, 0.8, 1);
}

.slide-fade-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateX(500px);
  opacity: 0;
}
</style>
