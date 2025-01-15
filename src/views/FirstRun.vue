<template>
    <div class="w-full justify-center items-center flex flex-col">
        <div class="container justify-center items-center">
            <div class="card bg-neutral shadow mb-4">
                <div class="card-body">
                    <h2 class="card-title">{{ $t('ManualAddProvider') }}</h2>
                    <div className="card-actions justify-start">
                        <ProviderSelect class="fill-warning" v-on:click="() => router.push('/addSubsonic')"><v-icon class="fill-warning h-full" name="gi-submarine" />
                            Subsonic / OpenSubsonic</ProviderSelect>
                    </div>
                </div>
            </div>
        </div>
        <div class="container justify-center items-center">
            <div class="card bg-neutral shadow">
                <div class="card-body">
                    <h2 class="card-title">{{ $t('AddedProviders') }}</h2>
                    <div class="card-actions"
                        :class="addedProviders && addedProviders.length > 0 ? 'justify-start' : 'justify-center'">
                        <div v-if="addedProviders === null" class="">
                            <span className="loading loading-spinner"></span>
                        </div>
                        <div v-else-if="addedProviders.length === 0">
                            <p>{{ $t('NoMediaProvidersAdded') }}</p>
                        </div>
                        <div v-else>
                            <ul class="flex">
                                <li v-for="item in addedProviders" v-bind:key="item.id" class="bg-base-100 rounded px-4">
                                    <div class="flex">
                                        <div class="pe-2 flex m-auto w-10">
                                            <v-icon name="gi-submarine" class="text-xl w-full h-full fill-warning" />
                                        </div>
                                        <div>
                                            <div>
                                                <h2 class="">{{ ProviderType[item.connection_type]}}</h2>
                                            </div>
                                            <div>
                                                <p>{{ item.ip }}</p>
                                            </div>
                                        </div>
                                    </div>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div class="fixed bottom-0 right-0 p-4">
            <button v-on:click="() => addedProviders?.length === 0 ? {} : router.push('/home')" :disabled="addedProviders?.length === 0" class="btn btn-primary">{{ $t('Done') }}</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import ProviderSelect from '../components/ProviderSelect.vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { Provider } from '../types/provider';
import { ProviderType } from '../types/providerType'

const router = useRouter();

const addedProviders = ref<Provider[] | null>(null);

onMounted(() => {
    invoke<Provider[]>("get_providers").then(providers => {
        addedProviders.value = providers
    })
})
</script>