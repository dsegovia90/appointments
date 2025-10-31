<script setup lang="ts">
import type { AuthFormField, FormSubmitEvent } from "@nuxt/ui";
import { z } from "zod";
import { useUserStore } from "~/stores/user";

definePageMeta({
  layout: "client-facing",
});

const loading = ref(true);
const route = useRoute();
const toast = useToast();
const userStore = useUserStore();
const token = ref<string | null>(null);

onMounted(() => {
  token.value = route.query.token as string | null;
  loading.value = false;
});

const resetFields = ref<AuthFormField[]>([
  {
    name: "password",
    type: "password",
    label: "New Password",
  },
  {
    name: "confirmPassword",
    type: "password",
    label: "Confirm New Password",
  },
]);

const resetSchema = z
  .object({
    password: z
      .string("New Password is required.")
      .min(6, "Minimum 6 characters.")
      .max(100),
    confirmPassword: z.string("Confirm New Password is required."),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords do not match.",
    path: ["confirmPassword"],
  });

type ResetSchema = z.output<typeof resetSchema>;

const handlePasswordChangeSubmit = async (
  values: FormSubmitEvent<ResetSchema>,
) => {
  if (!token.value) {
    toast.add({
      title: "Error",
      description: "Token is missing",
    });
    return;
  }
  await userStore.handlePasswordChange({ token: token.value, ...values.data });
};

const forgotFields = ref<AuthFormField[]>([
  {
    name: "email",
    type: "text",
    label: "Email",
  },
]);

const forgotSchema = z.object({
  email: z.email("Invalid email address."),
});

type ForgotSchema = z.output<typeof forgotSchema>;

const handleForgotSubmit = async (values: FormSubmitEvent<ForgotSchema>) => {
  await userStore.handlePasswordReset(values.data);
};
</script>

<template>
  <main class="flex items-center min-h-dvh">
    <UProgress v-if="loading" />
    <template v-else>
      <UCard class="max-w-md sm:min-w-md mx-auto">
        <UAuthForm
          v-if="token"
          title="Password Reset"
          description="Change your password."
          icon="i-lucide-user-pen"
          :fields="resetFields"
          :schema="resetSchema"
          @submit="handlePasswordChangeSubmit"
        >
          <template #footer>
            <CardFooterLink
              class="mb-1"
              text="Already have an account?"
              to="/login"
              link-text="Login"
            />
          </template>
        </UAuthForm>
        <UAuthForm
          v-else
          title="Password Reset"
          description="We'll send you an email with a link to reset your password."
          icon="i-lucide-user-search"
          :fields="forgotFields"
          :schema="forgotSchema"
          @submit="handleForgotSubmit"
        >
          <template #footer>
            <CardFooterLink
              class="mb-1"
              text="Already have an account?"
              to="/login"
              link-text="Login"
            />
          </template>
        </UAuthForm>
      </UCard>
    </template>
  </main>
</template>
