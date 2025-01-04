<template>
    <div><span class="loading loading-spinner"></span></div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const router = useRouter();
const route = useRoute();

onMounted(() => {
    invoke<boolean>('is_first_run').then(isFirstRun => {
        if(isFirstRun) {
            if(route.path !== '/firstRun') {
                router.push('/firstRun');
                return;
            } else {
                return;
            }
        }

        router.push('/home');
    });
})
</script>