<script setup lang="ts">
import CardFooterLink from "@/components/CardFooterLink.vue";
import type { AuthFormField, FormSubmitEvent } from "@nuxt/ui";
import { z } from "zod";
import { useUserStore } from "~/stores/user";

definePageMeta({
  layout: "client-facing",
});

const userStore = useUserStore();
const magicLinkAuthForm = useTemplateRef("magicLinkAuthForm");

const tabItems = [
  {
    label: "Password",
    icon: "i-lucide-user",
    slot: "password",
  },
  {
    label: "Magic Link",
    icon: "lucide:wand-sparkles",
    slot: "magic-link",
  },
];

const passwordFields = ref<AuthFormField[]>([
  {
    name: "email",
    type: "text",
    label: "Email",
  },
  {
    name: "password",
    type: "password",
    label: "Password",
  },
]);

const passwordSchema = z.object({
  email: z.email("Invalid email address."),
  password: z
    .string("Password is required.")
    .min(6, "Minimum 6 characters.")
    .max(100),
});

type PasswordSchema = z.output<typeof passwordSchema>;

const onPasswordSubmit = async (values: FormSubmitEvent<PasswordSchema>) => {
  // Handle form submission
  userStore.handleLogin(values.data);
};

const magicLinkFields = ref<AuthFormField[]>([
  {
    name: "email",
    type: "text",
    label: "Email",
  },
]);

const magicLinkSchema = z.object({
  email: z.email("Invalid email address."),
});

type MagicLinkSchema = z.output<typeof magicLinkSchema>;

const onMagicLinkSubmit = async (values: FormSubmitEvent<MagicLinkSchema>) => {
  // Handle form submission
  userStore.handleMagicLink(values.data, () => {
    if (magicLinkAuthForm.value) {
      magicLinkAuthForm.value.state.email = "";
    }
  });
};
</script>

<template>
  <main class="flex items-center min-h-dvh">
    <UCard class="max-w-md sm:min-w-md mx-auto">
      <UTabs :items="tabItems">
        <template #password>
          <UAuthForm
            title="Login"
            description="Enter your credentials to access your account."
            icon="i-lucide-user"
            :fields="passwordFields"
            :schema="passwordSchema"
            :submit="{ label: 'Submit' }"
            @submit="onPasswordSubmit"
          >
            <template #footer>
              <CardFooterLink
                class="mb-1"
                text="Not registered yet?"
                to="/register"
                link-text="Register"
              />
              <CardFooterLink
                text="Forgot Password?"
                to="/forgot"
                link-text="Recover Password"
              />
            </template>
          </UAuthForm>
        </template>
        <template #magic-link>
          <UAuthForm
            ref="magicLinkAuthForm"
            title="Magic Link"
            description="Enter your email, and receive a magic link."
            icon="lucide:wand-sparkles"
            :fields="magicLinkFields"
            :schema="magicLinkSchema"
            :submit="{ label: 'Submit' }"
            @submit="onMagicLinkSubmit"
          >
            <template #footer>
              <CardFooterLink
                class="mb-1"
                text="Not registered yet?"
                to="/register"
                link-text="Register"
              />
              <CardFooterLink
                text="Forgot Password?"
                to="/forgot"
                link-text="Recover Password"
              />
            </template>
          </UAuthForm>
        </template>
      </UTabs>
    </UCard>
  </main>
</template>
