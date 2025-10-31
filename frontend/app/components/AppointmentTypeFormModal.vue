<script setup lang="ts">
import type { AppointmentType } from "~/bindings/AppointmentType";
import { z } from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";

interface Props {
  appointmentType?: AppointmentType | null;
  open: boolean;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  "update:open": [value: boolean];
  submit: [data: Schema];
}>();

const isEditMode = computed(() => !!props.appointmentType);

const schema = z.object({
  name: z
    .string()
    .min(1, "Name is required")
    .max(100, "Name must be less than 100 characters"),
  display_name: z
    .string()
    .min(1, "Display name is required")
    .max(100, "Display name must be less than 100 characters"),
  duration_in_minutes: z
    .number()
    .min(5, "Duration must be at least 5 minutes")
    .max(480, "Duration must be less than 8 hours"),
});

type Schema = z.output<typeof schema>;

const state = reactive<Schema>({
  name: "",
  display_name: "",
  duration_in_minutes: 30,
});

// Update form state when appointment type changes
watch(
  () => props.appointmentType,
  (newAppointmentType) => {
    if (newAppointmentType) {
      state.name = newAppointmentType.name;
      state.display_name = newAppointmentType.display_name;
      state.duration_in_minutes = newAppointmentType.duration_in_minutes;
    } else {
      state.name = "";
      state.display_name = "";
      state.duration_in_minutes = 30;
    }
  },
  { immediate: true },
);

const onSubmit = async (event: FormSubmitEvent<Schema>) => {
  emit("submit", event.data);
  emit("update:open", false);
};

const closeModal = () => {
  emit("update:open", false);
};
</script>

<template>
  <UModal
    :open="open"
    :title="isEditMode ? 'Edit Appointment Type' : 'Create Appointment Type'"
    :description="
      isEditMode
        ? 'Update the appointment type details'
        : 'Add a new appointment type to your system'
    "
    @update:open="emit('update:open', $event)"
  >
    <template #body>
      <UForm
        :schema="schema"
        :state="state"
        class="space-y-4"
        @submit="onSubmit"
      >
        <UFormField label="Name" name="name" required>
          <UInput
            v-model="state.name"
            placeholder="e.g., consultation_30min"
            icon="i-lucide-tag"
          />
        </UFormField>

        <UFormField label="Display Name" name="display_name" required>
          <UInput
            v-model="state.display_name"
            placeholder="e.g., 30 Minute Consultation"
            icon="i-lucide-type"
          />
        </UFormField>

        <UFormField
          label="Duration (minutes)"
          name="duration_in_minutes"
          required
        >
          <UInputNumber
            v-model="state.duration_in_minutes"
            :min="5"
            :max="480"
            placeholder="30"
            icon="i-lucide-clock"
          />
        </UFormField>

        <div class="flex justify-end gap-2 pt-4">
          <UButton
            type="button"
            color="neutral"
            variant="outline"
            @click="closeModal"
          >
            Cancel
          </UButton>
          <UButton type="submit" color="primary">
            {{ isEditMode ? "Update" : "Create" }}
          </UButton>
        </div>
      </UForm>
    </template>
  </UModal>
</template>
