<script setup lang="ts">
import * as z from "zod";
import type { FormSubmitEvent } from "@nuxt/ui";
import type { UserSettingsProps } from "~/bindings/UserSettingsProps";
import type { UserSettings } from "~/bindings/UserSettings";

const toast = useToast();
const now = useNow();
const form = useTemplateRef("form");

const from = computed(() => {
  if (!state.value.start_how_far_from_now) return null;
  const from = new Date(now.value);
  from.setTime(
    from.getTime() +
      daysHoursMinutesToMinutes(state.value.start_how_far_from_now) * 60 * 1000,
  );
  return from;
});
const to = computed(() => {
  if (!state.value.end_how_far_from_now) return null;
  const to = new Date(now.value);
  to.setTime(
    to.getTime() +
      daysHoursMinutesToMinutes(state.value.end_how_far_from_now) * 60 * 1000,
  );
  return to;
});

const schema: z.ZodType<UserSettingsProps> = z
  .object({
    start_how_far_from_now: z.object({
      days: z.number().min(0),
      hours: z.number().min(0),
      minutes: z.number().min(0),
    }),
    end_how_far_from_now: z.object({
      days: z.number().min(0),
      hours: z.number().min(0),
      minutes: z.number().min(0),
    }),
  })
  .refine(
    (data) =>
      daysHoursMinutesToMinutes(data.start_how_far_from_now) <
      daysHoursMinutesToMinutes(data.end_how_far_from_now),
    {
      path: ["global"],
      message: "Start time must be before end time",
    },
  );

const state = ref<Partial<UserSettingsProps>>({});

onMounted(async () => {
  try {
    const response = await api<UserSettings>("/api/user_settings");
    state.value.start_how_far_from_now = minutesToDaysHoursMinutes(
      response.start_how_far_from_now_in_minutes,
    );
    state.value.end_how_far_from_now = minutesToDaysHoursMinutes(
      response.end_how_far_from_now_in_minutes,
    );
  } catch (error) {
    toast.add({
      title: "Error",
      description: "Failed to load user settings.",
      color: "error",
    });
    throw error;
  }
});

async function onSubmit(event: FormSubmitEvent<UserSettingsProps>) {
  try {
    await api("/api/user_settings", { method: "POST", body: event.data });
    toast.add({
      title: "Success",
      description: "Availability window saved.",
      color: "success",
    });
  } catch (error) {
    toast.add({
      title: "Error",
      description: "Failed to submit availability window.",
      color: "error",
    });
    throw error;
  }
}

const globalError = ref<string | null>(null);

watch(
  state,
  (latestState) => {
    const parsedState = schema.safeParse(latestState);
    if (!parsedState.success) {
      const gError = parsedState.error.issues.find((i) => i.code === "custom");
      if (gError) {
        globalError.value = gError.message;
      }
    } else {
      globalError.value = null;
    }
  },
  { deep: true },
);
</script>

<template>
  <h2 class="text-2xl mx-auto">Availability Window</h2>
  <p class="text-sm">
    Set the time range when you are available for appointments.
  </p>
  <p v-if="from && to" class="text-sm">
    Currently: from
    <span class="font-bold">{{
      useDateFormat(from, "dddd YYYY-MM-DD HH:mm:ss")
    }}</span>
    to
    <span class="font-bold">{{
      useDateFormat(to, "dddd YYYY-MM-DD HH:mm:ss")
    }}</span>
  </p>
  <div
    v-if="state?.start_how_far_from_now === undefined"
    class="mt-4 space-y-4"
  >
    <USkeleton class="h-16 max-w-full w-md" />
    <USkeleton class="h-16 max-w-full w-md" />
    <USkeleton class="h-12 max-w-full w-24" />
  </div>
  <UForm
    v-else
    ref="form"
    :schema="schema"
    :state="state"
    class="mt-4 space-y-4"
    @submit="onSubmit"
  >
    <div class="grid grid-cols-1 md:grid-cols-2">
      <DaysHoursMinutesFormField
        v-model="state.start_how_far_from_now"
        :error-highlight="!!globalError"
        label="Start Time from Today"
        class="rounded-b-none md:rounded-b-md md:rounded-r-none"
      />
      <DaysHoursMinutesFormField
        v-model="state.end_how_far_from_now"
        :error-highlight="!!globalError"
        label="End Time from Today"
        class="rounded-t-none md:rounded-t-md md:rounded-l-none"
      />
    </div>

    <p v-if="globalError" class="text-error">{{ globalError }}</p>
    <UButton size="xl" type="submit"> Save </UButton>
  </UForm>
</template>
