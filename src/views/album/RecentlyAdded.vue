<template>
    <Container>
        <label class="input input-bordered flex w-full max-w-sm items-center gap-2">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="h-4 w-4 opacity-70">
                <path fill-rule="evenodd"
                    d="M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z"
                    clip-rule="evenodd" />
            </svg>
            <input type="text" class="grow" placeholder="Search" />
        </label>
        <Card :background-color="Color.Base200" shadow fullWidth>
            <CardBody>
                <h1>Recently Added Albums</h1>
                <p>Recently added albums</p>
            </CardBody>
        </Card>
    </Container>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import Card from '../../components/Card.vue';
import CardBody from '../../components/CardBody.vue';
import Container from '../../components/Container.vue'
import { Color } from '../../types/color';
import { invoke } from '@tauri-apps/api/core';

onMounted(() => {
    invoke('get_recently_added_albums_for_provider', { providerId: 1}).then((albums: []) => {
        console.log(albums);
    })
})
</script>