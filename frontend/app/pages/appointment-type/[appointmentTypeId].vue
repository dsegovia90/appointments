<script setup lang="ts">
import type { FormSubmitEvent } from "@nuxt/ui";
import type { AvailabilityWindow } from "@/bindings/AvailabilityWindow";
import type { BookingParams } from "@/bindings/BookingParams";
import type { DateValue } from "@internationalized/date";
import type { BookDay } from "~/bindings/BookDay";
import { z } from "zod";
import {
  getLocalTimeZone,
  now,
  parseAbsoluteToLocal,
} from "@internationalized/date";

definePageMeta({
  layout: "client-facing",
});

const appointmentTypeStore = useAppointmentTypeStore();
const route = useRoute();
const selectedDay = ref<DateValue>();
const selectedTime = ref<AvailabilityWindow | null>(null);
const appointmentTypeId = useRoute().params.appointmentTypeId as string;
const days = ref<Map<BookDay["day"], BookDay["availabilities"]>>(new Map());
const firstDay = ref<BookDay["day"]>();
const lastDay = ref<BookDay["day"]>();
const loading = ref(true);
const timeAvailabilities = computed(() => {
  return days.value.get(selectedDay.value?.toString() ?? "") ?? [];
});
const submitted = ref(false);
const toast = useToast();

const fetchDays = async () => {
  const response = await api<BookDay[]>(
    `/api/client-facing/availabilities/${route.params.appointmentTypeId}`,
  );
  if (response[0]) {
    firstDay.value = response[0].day;
  }
  if (response[response.length - 1]) {
    lastDay.value = response[response.length - 1]!.day;
  }

  days.value = new Map(
    response.map((item) => [
      useDateFormat(item.day, "YYYY-MM-DD").value,
      item.availabilities,
    ]),
  );
};

onMounted(async () => {
  await fetchDays();
  loading.value = false;
});

const isDateUnavailable = (date: DateValue) => {
  return !days.value.get(date.toString())?.length;
};

const handleSelectTime = (availability: AvailabilityWindow) => {
  selectedTime.value = availability;
};

const schema = z.object({
  booker_name: z.string("Name is required").min(2).max(100),
  booker_email: z.email("Invalid email address."),
  booker_phone: z.string("Phone is required").min(8).max(20),
});

type Schema = z.output<typeof schema>;

const state = reactive<Partial<Schema>>({
  booker_name: undefined,
  booker_email: undefined,
  booker_phone: undefined,
});

const onSubmit = async (event: FormSubmitEvent<Schema>) => {
  if (!selectedTime.value) return;
  try {
    await api<unknown, BookingParams>(
      `/api/client-facing/book/${appointmentTypeId}`,
      {
        method: "POST",
        body: {
          ...event.data,
          from: selectedTime.value.start,
          to: selectedTime.value.end,
        },
      },
    );
    submitted.value = true;
  } catch (error) {
    console.error("Booking failed:", error);
    toast.add({
      title: "Booking Failed",
      description: "Please try again later.",
      color: "warning",
    });
  }
};
</script>

<template>
  <div class="grid md:grid-cols-2 md:grid-rows-2 md:min-h-dvh py-20 gap-8">
    <LoadingLinear v-if="loading" class="col-span-2 row-span-2" />
    <div v-else-if="submitted" class="col-span-2 row-span-2 flex items-center">
      <UCard class="max-w-md sm:min-w-md mx-auto" variant="soft">
        <template #header>
          <h1 class="text-2xl font-bold text-center">Success!</h1>
        </template>
        <p class="text-xl">
          Your appointment has been booked for
          {{
            appointmentTypeStore.appointmentTypes.get(
              parseInt(appointmentTypeId),
            )?.duration_in_minutes
          }}
          minutes {{ useDateFormat(selectedDay?.toString(), "dddd MMMM Do") }}
          at
          {{ useDateFormat(selectedTime?.start, "hh:mm A") }}
        </p>
      </UCard>
    </div>
    <template v-else>
      <UCard :ui="{ body: 'flex flex-wrap gap-8 justify-center h-auto' }">
        <div class="flex-1">
          <h1>
            {{
              appointmentTypeStore.appointmentTypes.get(
                parseInt(appointmentTypeId),
              )?.duration_in_minutes
            }}
            minutes
          </h1>
        </div>
        <UCalendar
          v-if="!appointmentTypeStore.loading && days"
          v-model="selectedDay"
          size="xl"
          variant="outline"
          :update:model-value="selectedTime = null"
          :min-value="
            firstDay ? parseAbsoluteToLocal(firstDay) : now(getLocalTimeZone())
          "
          :max-value="
            lastDay ? parseAbsoluteToLocal(lastDay) : now(getLocalTimeZone())
          "
          :is-date-unavailable="isDateUnavailable"
        >
          <template #day="{ day }">
            <UChip
              :show="
                !!days.get(day.toString())?.length &&
                day.day !== selectedDay?.day
              "
              color="primary"
              size="3xs"
            >
              {{ day.day }}
            </UChip>
          </template>
        </UCalendar>
      </UCard>

      <InOutTransition>
        <div
          v-if="selectedDay"
          :key="selectedDay?.toString()"
          name="fade"
          class="flex flex-col justify-center items-center space-x-2 space-y-2 text-center"
        >
          <h2 class="text-2xl font-bold mb-4">
            {{ useDateFormat(selectedDay?.toString(), "dddd MMMM Do") }}
            <InOutTransition animate-width>
              <span v-if="selectedTime" :key="selectedTime?.start.toString()">
                at
                {{ useDateFormat(selectedTime?.start, "hh:mm A") }}
              </span>
            </InOutTransition>
          </h2>
          <div
            :style="{
              gridTemplateRows: `repeat(${Math.min(8, Math.ceil(timeAvailabilities.length / Math.ceil(timeAvailabilities.length / 8)))}, 1fr)`,
            }"
            class="grid grid-flow-col"
          >
            <UButton
              v-for="availability in timeAvailabilities"
              :key="availability.start"
              class="block mx-auto cursor-pointer"
              size="xl"
              variant="ghost"
              @click="handleSelectTime(availability)"
            >
              {{ useDateFormat(`${availability.start}`, "hh:mm A") }}
            </UButton>
          </div>
        </div>
      </InOutTransition>

      <InOutTransition>
        <div v-if="days.size === 0" class="flex items-center justify-center">
          <p class="text-center">Sorry, there is currently no availability.</p>
        </div>
      </InOutTransition>

      <UForm
        v-if="selectedTime"
        :schema="schema"
        :state="state"
        class="md:col-span-2 space-y-4 max-w-md w-full mx-auto"
        @submit="onSubmit"
      >
        <UFormField label="Name" name="booker_name" size="xl" required>
          <UInput v-model="state.booker_name" class="w-full" />
        </UFormField>

        <UFormField label="Email" name="booker_email" size="xl" required>
          <UInput v-model="state.booker_email" type="email" class="w-full" />
        </UFormField>

        <UFormField label="Phone" name="booker_phone" size="xl" required>
          <UInput v-model="state.booker_phone" class="w-full" />
        </UFormField>

        <UButton type="submit" size="xl" block class="mt-4 cursor-pointer">
          Submit
        </UButton>
      </UForm>
    </template>
  </div>
</template>
