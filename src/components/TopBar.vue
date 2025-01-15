<template>
  <nav class="w-full navbar sticky top-0 bg-base-200 shadow-lg z-10">
    <span class="tooltip tooltip-bottom before:text-xs before:content-[attr(data-tip)]" data-tip="Toggle"><label aria-label="Open menu" for="drawer" class="btn btn-square btn-ghost drawer-button  "><svg width="20" height="20" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block h-5 w-5 stroke-current md:h-6 md:w-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path></svg></label></span>
    <h1>{{ breadcrumb }}</h1>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';


const route = useRoute();

const breadcrumb = computed(() =>   (route.meta.breadcrumb as string[]).join(' > '));
</script>