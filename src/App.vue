<template>
  <div class="min-h-dvh" :class="route.meta.fullCenter ? 'flex justify-center items-center' : ''">
    <router-view name="topBar">
    </router-view>
    <div class="drawer drawer-open pt-4">
      <router-view name="sidebar" class="me-4">
      </router-view>
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </div>
    <router-view name="player">
    </router-view>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute();
</script>
