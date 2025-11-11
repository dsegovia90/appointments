<script setup lang="ts">
import type { DaysHoursMinutes } from "@/bindings/DaysHoursMinutes";
import { z } from "zod";

interface Props {
  label: string;
  errorHighlight: boolean;
}

defineProps<Props>();

const model = defineModel<DaysHoursMinutes>({
  default: () => ({ days: 0, hours: 0, minutes: 0 }),
});

const schema: z.ZodType<DaysHoursMinutes> = z.object({
  days: z
    .number()
    .min(0, "Days must be non-negative")
    .max(365, "Days must be less than 365"),
  hours: z
    .number()
    .min(0, "Hours must be non-negative")
    .max(23, "Hours must be less than 24"),
  minutes: z
    .number()
    .min(0, "Minutes must be non-negative")
    .max(59, "Minutes must be less than 60"),
});
</script>

<template>
  <UCard variant="subtle">
    <template #header>
      <p class="text-2xl mx-auto">{{ label }}</p>
    </template>
    <UForm
      :schema="schema"
      :state="model"
      class="grid grid-cols-1 lg:grid-cols-3 gap-2"
      size="xl"
    >
      <UFormField
        name="days"
        :ui="{ label: 'text-lg lg:text-2xl' }"
        :label="`${model.days} Day${model.days === 1 ? '' : 's'}`"
        :error="errorHighlight"
      >
        <UInputNumber v-model="model.days" :max="365" />
      </UFormField>
      <UFormField
        name="hours"
        :ui="{ label: 'text-lg lg:text-2xl' }"
        :label="`${model.hours} Hour${model.hours === 1 ? '' : 's'}`"
        :error="errorHighlight"
      >
        <UInputNumber v-model="model.hours" :min="0" :max="23" />
      </UFormField>
      <UFormField
        name="minutes"
        :ui="{ label: 'text-lg lg:text-2xl' }"
        :label="`${model.minutes} Minute${model.minutes === 1 ? '' : 's'}`"
        :error="errorHighlight"
      >
        <UInputNumber v-model="model.minutes" :min="0" :max="59" />
      </UFormField>
    </UForm>
  </UCard>
</template>
