<script setup lang="ts">
import type { AuthFormField, FormSubmitEvent } from "@nuxt/ui";
import { z } from "zod";
import { useUserStore } from "~/stores/user";

definePageMeta({
  layout: "client-facing",
});

const userStore = useUserStore();

const fields = ref<AuthFormField[]>([
  {
    name: "email",
    type: "email",
    label: "Email",
  },
  {
    name: "name",
    type: "text",
    label: "Name",
  },
  {
    name: "password",
    type: "password",
    label: "Password",
  },
  {
    name: "confirmPassword",
    type: "password",
    label: "Confirm Password",
  },
]);

const schema = z
  .object({
    email: z.email("Invalid email address."),
    name: z.string("Name is required."),
    password: z
      .string("Password is required.")
      .min(6, "Minimum 6 characters.")
      .max(100),
    confirmPassword: z.string("Confirm password is required."),
  })
  .refine((data) => data.password === data.confirmPassword, {
    message: "Passwords do not match.",
    path: ["confirmPassword"],
  });

type Schema = z.output<typeof schema>;

const onSubmit = async (values: FormSubmitEvent<Schema>) => {
  // Handle form submission
  userStore.handleRegister(values.data);
};
</script>

<template>
  <main class="flex items-center min-h-dvh">
    <UCard class="max-w-md sm:min-w-md mx-auto">
      <UAuthForm
        title="Register"
        description="Create a new account."
        icon="i-lucide-user"
        :fields="fields"
        :schema="schema"
        @submit="onSubmit"
      >
        <template #footer>
          <CardFooterLink
            class="mb-1"
            text="Already have an account?"
            to="/login"
            link-text="Login"
          />
          <CardFooterLink
            text="Forgot Password?"
            to="/forgot"
            link-text="Recover Password"
          />
        </template>
      </UAuthForm>
    </UCard>
  </main>
</template>
