<script setup lang="ts">
import type { AppointmentType } from "~/bindings/AppointmentType";
import type { FormSubmitEvent, TableColumn } from "@nuxt/ui";
import { z } from "zod";

const appointmentTypesStore = useAppointmentTypesAdminStore();

const isFormModalOpen = ref(false);
const isDeleteModalOpen = ref(false);

const formSchema = z.object({
  display_name: z
    .string()
    .min(1, "Display name is required")
    .max(100, "Display name must be less than 100 characters"),
  duration_in_minutes: z
    .number()
    .min(5, "Duration must be at least 5 minutes")
    .max(480, "Duration must be less than 8 hours"),
});

type FormSchema = z.output<typeof formSchema>;

const formState = reactive<FormSchema>({
  display_name: "",
  duration_in_minutes: 30,
});

// Open create modal
const handleCreate = () => {
  appointmentTypesStore.selectAppointmentType(null);
  // Reset form state for new appointment type
  Object.assign(formState, {
    display_name: "",
    duration_in_minutes: 30,
  });
  isFormModalOpen.value = true;
};

// Open edit modal
const handleEdit = (appointmentType: AppointmentType) => {
  appointmentTypesStore.selectAppointmentType(appointmentType);
  // Populate form with existing data
  Object.assign(formState, {
    name: appointmentType.name,
    display_name: appointmentType.display_name,
    duration_in_minutes: appointmentType.duration_in_minutes,
  });
  isFormModalOpen.value = true;
};

// Open delete modal
const handleDelete = (appointmentType: AppointmentType) => {
  appointmentTypesStore.selectAppointmentType(appointmentType);
  isDeleteModalOpen.value = true;
};

// Submit form (create or update)
const handleFormSubmit = async (event: FormSubmitEvent<FormSchema>) => {
  if (appointmentTypesStore.selectedAppointmentType) {
    // Update existing appointment type
    await appointmentTypesStore.updateAppointmentType(
      appointmentTypesStore.selectedAppointmentType.id,
      event.data,
    );
  } else {
    // Create new appointment type
    await appointmentTypesStore.createAppointmentType(event.data);
  }
  isFormModalOpen.value = false;
};

// Confirm delete
const handleDeleteConfirm = async () => {
  if (!appointmentTypesStore.selectedAppointmentType) return;

  await appointmentTypesStore.deleteAppointmentType(
    appointmentTypesStore.selectedAppointmentType.id,
  );
  isDeleteModalOpen.value = false;
  appointmentTypesStore.selectAppointmentType(null);
};

// Table columns configuration
const columns: TableColumn<AppointmentType>[] = [
  {
    accessorKey: "name",
    header: "Name",
  },
  {
    accessorKey: "display_name",
    header: "Display Name",
  },
  {
    accessorKey: "duration_in_minutes",
    header: "Duration",
  },
  {
    accessorKey: "created_at",
    header: "Created",
  },
  {
    id: "actions",
    header: "Actions",
  },
];

// Refresh data
const handleRefresh = () => {
  appointmentTypesStore.fetchAppointmentTypes();
};

// Load data on mount
onMounted(() => {
  appointmentTypesStore.fetchAppointmentTypes();
});
</script>

<template>
  <UDashboardPanel id="appointment-types">
    <template #header>
      <UDashboardNavbar title="Appointment Types">
        <template #leading>
          <UDashboardSidebarCollapse />
        </template>
      </UDashboardNavbar>
    </template>

    <!-- Content -->
    <template #body>
      <p>Manage your appointment types and their durations</p>
      <div class="ml-auto flex gap-2">
        <UButton
          icon="i-lucide-refresh-cw"
          color="neutral"
          variant="outline"
          :loading="appointmentTypesStore.loading"
          :disabled="appointmentTypesStore.loading"
          @click="handleRefresh"
        >
          Refresh
        </UButton>
        <UButton icon="i-lucide-plus" @click="handleCreate">
          Add Appointment Type
        </UButton>
      </div>
      <UCard
        v-if="
          appointmentTypesStore.loading &&
          appointmentTypesStore.appointmentTypes.length === 0
        "
      >
        <div
          v-if="
            appointmentTypesStore.loading &&
            appointmentTypesStore.appointmentTypes.length === 0
          "
          class="flex justify-center items-center h-64"
        >
          <UIcon
            name="i-lucide-loader-2"
            class="animate-spin h-8 w-8 text-primary"
          />
        </div>

        <div
          v-else-if="appointmentTypesStore.appointmentTypes.length === 0"
          class="text-center py-12"
        >
          <UIcon
            name="i-lucide-calendar-off"
            class="h-12 w-12 text-neutral-100 mx-auto mb-4"
          />
          <h3 class="text-lg font-medium mb-2">No appointment types found</h3>
          <p class="text-sm text-neutral-100 mb-4">
            Get started by creating your first appointment type
          </p>
          <UButton icon="i-lucide-plus" @click="handleCreate">
            Create Appointment Type
          </UButton>
        </div>
      </UCard>

      <!-- Table view -->
      <UTable
        v-else
        class="border border-muted rounded-lg"
        :columns="columns"
        :data="appointmentTypesStore.appointmentTypes"
        :loading="appointmentTypesStore.loading"
      >
        <template #name-cell="{ row }">
          <div class="font-medium">{{ row.original.name }}</div>
        </template>

        <template #display_name-cell="{ row }">
          <div>{{ row.original.display_name }}</div>
        </template>

        <template #duration_in_minutes-cell="{ row }">
          <UBadge color="neutral" variant="subtle">
            <UIcon name="lucide:clock" class="w-3 h-3 mr-1" />
            {{ formatDuration(row.original.duration_in_minutes) }}
          </UBadge>
        </template>

        <template #created_at-cell="{ row }">
          <span class="text-sm text-neutral-100">
            {{ formatDate(row.original.created_at) }}
          </span>
        </template>

        <template #actions-cell="{ row }">
          <div class="flex items-center justify-end gap-2">
            <UButton
              icon="i-lucide-edit"
              size="sm"
              color="neutral"
              variant="ghost"
              @click="handleEdit(row.original)"
            />
            <UButton
              icon="i-lucide-trash"
              size="sm"
              color="error"
              variant="ghost"
              @click="handleDelete(row.original)"
            />
          </div>
        </template>
      </UTable>
      <!-- Form Modal -->
      <UModal
        v-model:open="isFormModalOpen"
        :title="
          appointmentTypesStore.selectedAppointmentType
            ? 'Edit Appointment Type'
            : 'Create Appointment Type'
        "
        :description="
          appointmentTypesStore.selectedAppointmentType
            ? 'Update the appointment type details'
            : 'Add a new appointment type to your system'
        "
      >
        <template #body>
          <UForm
            :schema="formSchema"
            :state="formState"
            class="space-y-4"
            @submit="handleFormSubmit"
          >
            <UFormField label="Display Name" name="display_name" required>
              <UInput
                v-model="formState.display_name"
                placeholder="e.g., 30 Minute Consultation"
                icon="i-lucide-type"
              />
            </UFormField>

            <UFormField
              label="Duration (minutes)"
              name="duration_in_minutes"
              required
            >
              <UInput
                v-model.number="formState.duration_in_minutes"
                type="number"
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
                @click="
                  () => {
                    isFormModalOpen = false;
                  }
                "
              >
                Cancel
              </UButton>
              <UButton type="submit" :loading="appointmentTypesStore.loading">
                {{
                  appointmentTypesStore.selectedAppointmentType
                    ? "Update"
                    : "Create"
                }}
              </UButton>
            </div>
          </UForm>
        </template>
      </UModal>

      <!-- Delete Confirmation Modal -->
      <UModal
        v-model:open="isDeleteModalOpen"
        title="Delete Appointment Type"
        description="Are you sure you want to delete this appointment type?"
      >
        <template #body>
          <div
            v-if="appointmentTypesStore.selectedAppointmentType"
            class="space-y-4"
          >
            <UAlert
              icon="i-lucide-alert-triangle"
              color="warning"
              title="Warning"
              variant="soft"
              description="This action cannot be undone. This will permanently delete the appointment type."
            />

            <div class="bg-muted rounded-lg p-4 space-y-2">
              <div class="flex justify-between">
                <span class="text-sm text-muted">Name:</span>
                <span class="text-sm font-medium">
                  {{ appointmentTypesStore.selectedAppointmentType.name }}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-muted">Display Name:</span>
                <span class="text-sm font-medium">
                  {{
                    appointmentTypesStore.selectedAppointmentType.display_name
                  }}
                </span>
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-muted">Duration:</span>
                <span class="text-sm font-medium">
                  {{
                    appointmentTypesStore.selectedAppointmentType
                      .duration_in_minutes
                  }}
                  minutes
                </span>
              </div>
            </div>
          </div>
        </template>

        <template #footer>
          <div class="flex justify-end gap-2">
            <UButton
              color="neutral"
              variant="outline"
              @click="
                () => {
                  isDeleteModalOpen = false;
                }
              "
            >
              Cancel
            </UButton>
            <UButton
              color="error"
              :loading="appointmentTypesStore.loading"
              @click="handleDeleteConfirm"
            >
              Delete
            </UButton>
          </div>
        </template>
      </UModal>
    </template>
  </UDashboardPanel>
</template>
