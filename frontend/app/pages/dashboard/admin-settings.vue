<script setup lang="ts">
import type { AdminSettings } from "~/bindings/AdminSettings";
import { useAdminSettingsAPI } from "~/composables/useAdminSettingsAPI";

const adminSettings = ref<AdminSettings>();
const { fetch, update } = useAdminSettingsAPI();
const showGoogleCloudApiKey = ref(false);

onMounted(async () => {
  adminSettings.value = await fetch();
});

const handleUpdate = async (data: Partial<AdminSettings>) => {
  adminSettings.value = await update(data);
};
</script>

<template>
  <div class="space-y-6">
    <UPageHeader>
      <template #title>
        <h1 class="text-4xl font-bold">Admin Settings</h1>
      </template>
      <template #description>
        <p class="text-sm text-neutral-100 mt-1">
          Update your admin settings and preferences
        </p>
      </template>
    </UPageHeader>

    <UPageBody>
      <LoadingLinear v-if="!adminSettings"> Loading... </LoadingLinear>
      <template v-else>
        <h2 class="text-2xl font-bold">General Settings</h2>
        <UFormField
          label="Allow new users to register?"
          description="They will register without admin privileges"
          size="xl"
        >
          <USwitch
            v-model="adminSettings.allow_new_registrations"
            unchecked-icon="i-lucide-x"
            checked-icon="i-lucide-check"
            default-value
            :label="
              adminSettings.allow_new_registrations!!
                ? 'Allow new registrations'
                : 'Disallow new registrations'
            "
            @update:model-value="
              (value) => handleUpdate({ allow_new_registrations: value })
            "
          />
        </UFormField>

        <USeparator />

        <h2 class="text-2xl font-bold">Notification Settings</h2>

        <div class="space-y-4">
          <UFormField
            label="Google API Key"
            description="Once set, the field will be hidden."
            size="xl"
          >
            <UInput
              v-model="adminSettings.google_cloud_api_key"
              class="min-w-120"
              size="xl"
              :type="showGoogleCloudApiKey ? 'text' : 'password'"
            >
              <template
                v-if="adminSettings.google_cloud_api_key?.length"
                #trailing
              >
                <UButton
                  color="neutral"
                  variant="link"
                  size="sm"
                  icon="i-lucide-circle-x"
                  aria-label="Clear input"
                  @click="
                    handleUpdate({
                      google_cloud_api_key: '',
                    })
                  "
                />
                <UButton
                  color="neutral"
                  variant="link"
                  size="sm"
                  :icon="
                    showGoogleCloudApiKey ? 'i-lucide-eye-off' : 'i-lucide-eye'
                  "
                  :aria-label="
                    showGoogleCloudApiKey ? 'Hide password' : 'Show password'
                  "
                  :aria-pressed="showGoogleCloudApiKey"
                  aria-controls="password"
                  @click="showGoogleCloudApiKey = !showGoogleCloudApiKey"
                />
              </template>
            </UInput>
          </UFormField>
          <UButton
            label="Save"
            size="xl"
            @click="
              handleUpdate({
                google_cloud_api_key: adminSettings.google_cloud_api_key,
              })
            "
          />
        </div>
      </template>
    </UPageBody>
  </div>
</template>
