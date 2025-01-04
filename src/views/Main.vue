<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main class="flex m-0 pt-[10vh] flex-col justify-center text-center">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="flex justify-center">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="h-[6em] p-[1.5em] transition-[0.75s] vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="h-[6em] p-[1.5em] transition-[0.75s] hover:drop-shadow-sm" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="../assets/vue.svg" class="h-[6em] p-[1.5em] transition-[0.75s] vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="flex justify-center" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button class="btn" type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>
