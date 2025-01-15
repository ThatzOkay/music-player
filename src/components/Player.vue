<template>
    <div class="w-full bottom-0 fixed">
        <a-player class="m-0 bg-base-200 shadow-lg" ref="player" />
    </div>
</template>
<script setup lang="ts">
import { APlayer } from '@thatzokay/vue-aplayer';
import { ref } from 'vue';

const player = ref<typeof APlayer | null>(null);

</script>