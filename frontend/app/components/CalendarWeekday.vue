<script setup lang="ts">
import type { Weekday } from "~/utils/constants";
import { weekdays } from "~/utils/constants";

const AVAILABILITY_WINDOW_HEIGHT = 60;
interface Props {
  weekday: Weekday;
}
const props = defineProps<Props>();
const weekdayIndex = weekdays.findIndex((day) => day === props.weekday);
const weeklyAvailabilityStore = useWeeklyAvailabilityStore();
const weekdayAvailability = computed(
  () => weeklyAvailabilityStore.availabilityWeek?.[props.weekday.key],
);
const target = useTemplateRef("target");
const { elementY, isOutside } = useMouseInElement(target);
const newTop = computed(() =>
  windowCalculator({
    min: 0,
    max: 1440 - AVAILABILITY_WINDOW_HEIGHT,
    value: elementY.value - AVAILABILITY_WINDOW_HEIGHT / 2,
  }),
);
const newBottom = computed(() =>
  windowCalculator({
    min: AVAILABILITY_WINDOW_HEIGHT,
    max: 1440,
    value: elementY.value + AVAILABILITY_WINDOW_HEIGHT / 2,
  }),
);
const clasherChecker = computed(() => {
  return !!weekdayAvailability.value?.find((item) => {
    return (
      (newTop.value >= item.normalized.from &&
        newTop.value <= item.normalized.to) ||
      (newBottom.value >= item.normalized.from &&
        newBottom.value <= item.normalized.to) ||
      (newTop.value <= item.normalized.from &&
        newBottom.value >= item.normalized.to)
    );
  });
});

const handleCreate = () => {
  weeklyAvailabilityStore.handleCreate({
    normalized: { from: newTop.value, to: newBottom.value },
    weekday: weekdayIndex,
  });
};
</script>

<template>
  <div ref="target" class="h-[1440px] relative">
    <div
      v-if="
        !isOutside && !clasherChecker && !weeklyAvailabilityStore.isUpdating
      "
      :style="{
        top: `${newTop}px`,
        height: `${AVAILABILITY_WINDOW_HEIGHT}px`,
        left: '0',
      }"
      class="bg-neutral-200 flex-col absolute w-full flex justify-between items-center p-2 rounded"
      @click="handleCreate"
    >
      <p class="relative z-10 text-black font-strong">
        {{ getAvailabilityTime(newTop) }}
      </p>

      <p class="relative z-10 text-black font-strong">
        {{ getAvailabilityTime(newBottom) }}
      </p>
    </div>

    <template v-if="weekdayAvailability">
      <AvailabilityWindow
        v-for="availability in weekdayAvailability"
        :key="availability.model.id"
        :availability="availability"
        :weekday="weekdayIndex"
        @delete.once="weeklyAvailabilityStore.handleRemove"
        @update="weeklyAvailabilityStore.handleUpdate"
      />
    </template>
  </div>
</template>
