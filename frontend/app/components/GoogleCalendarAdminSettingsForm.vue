<script setup lang="ts">
import * as z from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";
import type { GoogleCalendarSettings } from "~/bindings/GoogleCalendarSettings";

// Define props
interface Props {
  initialValues?: GoogleCalendarSettings;
  showReset?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  initialValues: () => ({
    google_calendar_api_key: "",
    google_oauth_client_id: "",
    google_oauth_secret: "",
    google_oauth_redirect_uri_base: "",
  }),
  showReset: false,
});

// Define emits
const emit = defineEmits<{
  submit: [data: GoogleCalendarSettings];
  reset: [];
}>();

// Validation schema using Zod
const schema = z.object({
  google_calendar_api_key: z
    .string()
    .min(1, "API key is required")
    .regex(/^[A-Za-z0-9-_]+$/, "Invalid API key format"),

  google_oauth_client_id: z
    .string()
    .min(1, "OAuth client ID is required")
    .regex(
      /^\d+-[a-z0-9]+\.apps\.googleusercontent\.com$/,
      "Invalid OAuth client ID format (should end with .apps.googleusercontent.com)",
    ),
  google_oauth_secret: z
    .string()
    .min(1, "OAuth secret is required")
    .refine((value) => !value.includes("*"), "OAuth secret cannot contain *"),
  google_oauth_redirect_uri_base: z
    .url("Must be a valid URL")
    .refine(
      (url) => url.startsWith("https://") || url.startsWith("http://localhost"),
      "URL must use HTTPS (or http://localhost for development)",
    ),
});

type Schema = z.infer<typeof schema>;

// Form state
const state = reactive<Partial<GoogleCalendarSettings>>({
  google_calendar_api_key: props.initialValues?.google_calendar_api_key || "",
  google_oauth_client_id: props.initialValues?.google_oauth_client_id || "",
  google_oauth_secret: props.initialValues?.google_oauth_secret || "",
  google_oauth_redirect_uri_base:
    props.initialValues?.google_oauth_redirect_uri_base || "",
});

// Loading state
const isSubmitting = ref(false);

// Toast for notifications
const toast = useToast();

// Handle form submission
async function onSubmit(event: FormSubmitEvent<Schema>) {
  isSubmitting.value = true;

  try {
    // Emit the validated data
    emit("submit", event.data);

    // Show success toast
    toast.add({
      title: "Settings Saved",
      description:
        "Your Google Calendar settings have been updated successfully.",
      color: "success",
    });
  } catch (error) {
    // Show error toast
    toast.add({
      title: "Error",
      description:
        error instanceof Error ? error.message : "Failed to save settings",
      color: "error",
    });
  } finally {
    isSubmitting.value = false;
  }
}

// Handle reset
function onReset() {
  // Reset to initial values
  state.google_calendar_api_key =
    props.initialValues?.google_calendar_api_key || "";
  state.google_oauth_client_id =
    props.initialValues?.google_oauth_client_id || "";
  state.google_oauth_secret = props.initialValues?.google_oauth_secret || "";
  state.google_oauth_redirect_uri_base =
    props.initialValues?.google_oauth_redirect_uri_base || "";

  emit("reset");

  toast.add({
    title: "Form Reset",
    description: "Settings have been reset to their original values.",
    color: "info",
  });
}

// Watch for external changes to initial values
watch(
  () => props.initialValues,
  (newValues) => {
    if (newValues) {
      state.google_calendar_api_key =
        newValues.google_calendar_api_key || state.google_calendar_api_key;
      state.google_oauth_client_id =
        newValues.google_oauth_client_id || state.google_oauth_client_id;
      state.google_oauth_secret =
        newValues.google_oauth_secret || state.google_oauth_secret;
      state.google_oauth_redirect_uri_base =
        newValues.google_oauth_redirect_uri_base ||
        state.google_oauth_redirect_uri_base;
    }
  },
  { deep: true },
);
</script>

<template>
  <UForm :schema="schema" :state="state" class="space-y-4" @submit="onSubmit">
    <UFormField
      size="xl"
      name="google_calendar_api_key"
      label="Google Calendar API Key"
      description="Your Google Calendar API key for accessing calendar services"
      required
    >
      <UInput
        v-model="state.google_calendar_api_key"
        class="w-full md:max-w-120"
        type="password"
        placeholder="Enter your API key"
      />
    </UFormField>

    <UFormField
      size="xl"
      name="google_oauth_client_id"
      label="Google OAuth Client ID"
      description="The OAuth 2.0 client ID from your Google Cloud Console"
      required
    >
      <UInput
        v-model="state.google_oauth_client_id"
        class="w-full md:max-w-120"
        placeholder="Enter your OAuth client ID"
      />
    </UFormField>

    <UFormField
      size="xl"
      name="google_oauth_secret"
      label="Google OAuth Secret"
      description="The OAuth 2.0 client secret from your Google Cloud Console"
      required
    >
      <UInput
        v-model="state.google_oauth_secret"
        class="w-full md:max-w-120"
        type="password"
        placeholder="Enter your OAuth secret"
      />
    </UFormField>

    <UFormField
      size="xl"
      name="google_oauth_redirect_uri_base"
      label="OAuth Redirect URI Base"
      description="The base URI for OAuth redirects (e.g., https://yourdomain.com)"
      help="This should be the base URL where your application is hosted"
      required
    >
      <UInput
        v-model="state.google_oauth_redirect_uri_base"
        class="w-full md:max-w-120"
        type="url"
        placeholder="https://yourdomain.com"
      />
    </UFormField>

    <div class="flex gap-3">
      <UButton type="submit" :loading="isSubmitting"> Save Settings </UButton>
      <UButton
        v-if="showReset"
        variant="outline"
        color="neutral"
        @click="onReset"
      >
        Reset
      </UButton>
    </div>
  </UForm>
</template>
