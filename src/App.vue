<template>
  <div class="min-h-dvh flex flex-col justify-center items-center">
<router-view v-slot="{ Component }">
  <transition name="fade" mode="out-in">
    <component :is="Component" />
  </transition>
</router-view>
  <!-- <router-view /> -->
</div>
</template>

<script setup lang="ts">
</script>
