<template>
    <div>
        <h1>Home</h1>
        <p>Home page content</p>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onMounted } from 'vue';
import { Provider } from '../types/provider';


onMounted(() => {
    invoke<Provider[]>('get_providers').then(providers => {
        for (const provider of providers) {
            invoke('get_albums_for_provider', { providerId: provider.id }).then(albums => {
                console.log(albums);
            });
        }
    });
});
</script>