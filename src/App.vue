<script setup lang="ts">
import { emit } from '@tauri-apps/api/event';
import { ref } from 'vue';
import Search from './pages/search_result.vue';
const page = ref<string>("discover")
const isSearch = ref<boolean>(false)
const barrells = ref<Barrell[]>([])
const search = ref<string>("")

async function getBarrells() {
  barrells.value = await (await fetch("https://api.fermentpkg.tech/barrells")).json()
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

</script>

<template>
  <div class="container">
    <nav>
      <a :class="page=='discover'?'selected':null" @click="setPage('discover')">Discover</a>
      <a :class="page=='popular'?'selected':null" @click="setPage('popular')">Popular</a>
      <a :class="page=='install'?'selected':null" @click="setPage('install')">Installs</a>
      <div class="search">
        <input type="text" v-model="search" placeholder="Search for a package">
        <button @click="searchForPackage()">Search</button>
      </div>
    </nav>
    <div class="content">
      <Suspense v-if="!isSearch">
        <component :is="page"></component>

      </Suspense>
      <Suspense v-if="isSearch">
        <Search :barrells="filteredBarrells" />
      </Suspense>

    </div>
  </div>
</template>

<style scoped lang="scss">
$color: rgb(25, 26, 27);

.content {
  width: 80%;
  margin-left: 20%;
  background-color: $color;
  //get height of window
  height: 100vh;
  overflow-y: scroll;


}

.container {
  width: 100%;
  height: 0%;
  padding: 0;
}

nav {
  //set color variable
  margin: auto;

  //put navbar on left side
  background: none;
  position: fixed;
  top: 0;
  left: 0;
  width: 20%;
  height: 100%;
  padding: 20px 0;
  transition: all 0.5s ease;
  //make navbar have rounded corners
  border-radius: 0 20px 20px 0;
  margin-right: 0%;
  overflow: hidden;

  * {
    margin-bottom: 10px;
  }

  a {
    display: block;
    color: #f2f2f2;
    padding: 16px;
    text-decoration: none;
    cursor: pointer;
  }

  a.selected {
    background: #f2f2f2;
    color: $color;
  }

}
</style>
