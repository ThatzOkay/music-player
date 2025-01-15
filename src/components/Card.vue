<template>
    <div class="card" :class="classes" >
        <slot></slot>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Color } from '../types/color';


const props = defineProps<{
    backgroundColor: Color,
    shadow: boolean,
    fullWidth: boolean
}>();

const classes = ref(`bg-${props.backgroundColor} ${props.shadow ? 'shadow-lg' : ''} ${props.fullWidth ? 'w-full' : ''}`);

</script>
