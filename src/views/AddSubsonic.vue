<template>
    <div class="w-full flex flex-col">
        <div class="container justify-center m-auto items-center">
            <div class="card bg-neutral shadow">
                <div class="card-body">
                    <h1 class="card-title justify-center font-bold text-5xl"><v-icon name="gi-submarine"
                            class="w-14 h-14" />Subsonic</h1>
                    <Form class="flex flex-col justify-center items-center" :validationSchema="validationSchema"
                        @submit="handleSubmit">
                        <Field name="host" placeholder="host" class="input input-bordered w-full max-w-xs mb-2">
                        </Field>
                        <div class="w-[45%] mb-4">
                            <ErrorMessage name="host" class="text-secondary" />
                        </div>
                        <Field name="username" :placeholder="$t('Username')"
                            class="input input-bordered w-full max-w-xs mb-2"></Field>
                        <div class="w-[45%] mb-4">
                            <ErrorMessage name="username" class="text-secondary" />
                        </div>
                        <Field type="password" name="password" :placeholder="$t('Password')"
                            class="input input-bordered w-full max-w-xs mb-2"></Field>
                        <div class="w-[45%] mb-4">
                            <ErrorMessage name="password" class="text-secondary" />
                        </div>
                        <button type="submit" class="btn btn-primary">
                            <div v-if="!loading">{{ $t('Add') }}</div>
                            <div v-else><span class="loading loading-spinner"></span></div>
                        </button>
                    </Form>
                </div>
            </div>
        </div>
        <!-- Back button left bottom -->
        <div class="fixed bottom-0 left-0 p-4">
            <button class="btn btn-primary" @click="router.back()">{{ $t('Back') }}</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Form, Field, ErrorMessage } from 'vee-validate';
import { useRouter } from 'vue-router';
import { toTypedSchema } from '@vee-validate/yup';
import { InferType, object, string } from 'yup';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useToast } from 'vue-toastification';
import { useI18n } from 'vue-i18n';

const loading = ref(false);
const router = useRouter();
const toast = useToast();
const i18n = useI18n();

const schema = object({
    host: string().required(i18n.t("FieldRequired")),
    username: string().required(i18n.t("FieldRequired")),
    password: string().required(i18n.t("FieldRequired")),
});

const validationSchema = toTypedSchema(schema);

const handleSubmit = async (values: unknown, { resetForm }: { resetForm: () => void }) => {
    const formValues = values as InferType<typeof schema>;

    loading.value = true;

    if (!await invoke<boolean>("check_credentials", { provider: "Subsonic", host: formValues.host, username: formValues.username, password: formValues.password })) {
        toast.error(i18n.t('InvalidCredentials'));
        loading.value = false;
        return;
    }

    try {
        var addedProvider = await invoke("add_provider", { provider: "Subsonic", host: formValues.host, username: formValues.username, password: formValues.password });
    } catch (e) {
        console.error("Err response", e);
        toast.error("Error")
        loading.value = false;
        return;
    }

    console.log("Added", addedProvider);
    if (addedProvider) {
        resetForm();
        router.push('/firstRun');
    }
    loading.value = false;
};
</script>