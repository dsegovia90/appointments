<script setup lang="ts">
import type { AppointmentType } from "~/bindings/AppointmentType";

interface Props {
  appointmentType: AppointmentType | null;
  open: boolean;
}

defineProps<Props>();
const emit = defineEmits<{
  "update:open": [value: boolean];
  confirm: [];
}>();

const handleConfirm = () => {
  emit("confirm");
  emit("update:open", false);
};

const handleCancel = () => {
  emit("update:open", false);
};
</script>

<template>
  <UModal
    :open="open"
    title="Delete Appointment Type"
    description="Are you sure you want to delete this appointment type?"
    @update:open="emit('update:open', $event)"
  >
    <template #body>
      <div v-if="appointmentType" class="space-y-4">
        <UAlert
          icon="i-lucide-alert-triangle"
          color="warning"
          title="Warning"
          description="This action cannot be undone. This will permanently delete the appointment type."
        />

        <div class="bg-neutral-50 dark:bg-neutral-900 rounded-lg p-4 space-y-2">
          <div class="flex justify-between">
            <span class="text-sm text-muted">Name:</span>
            <span class="text-sm font-medium">{{ appointmentType.name }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-muted">Display Name:</span>
            <span class="text-sm font-medium">{{
              appointmentType.display_name
            }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-sm text-muted">Duration:</span>
            <span class="text-sm font-medium"
              >{{ appointmentType.duration_in_minutes }} minutes</span
            >
          </div>
        </div>
      </div>
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <UButton color="neutral" variant="outline" @click="handleCancel">
          Cancel
        </UButton>
        <UButton color="error" @click="handleConfirm"> Delete </UButton>
      </div>
    </template>
  </UModal>
</template>
