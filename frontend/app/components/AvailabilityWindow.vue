<script setup lang="ts">
import type { AvailabilityDay } from "~/bindings/AvailabilityDay";

const topResizer = useTemplateRef("topResizer");
const bottomResizer = useTemplateRef("bottomResizer");

const { pressed: resizingTop } = useMousePressed({ target: topResizer });
const { pressed: resizingBottom } = useMousePressed({ target: bottomResizer });
const waitingForTopUpdate = ref<number | null>(null);
const waitingForBottomUpdate = ref<number | null>(null);

const weeklyAvailabilityStore = useWeeklyAvailabilityStore();

interface Props {
  availability: AvailabilityDay;
  weekday: number;
}
const props = defineProps<Props>();

const grid = inject<HTMLDivElement>("grid");
const x = useMouseInElement(grid);

const computedTop = computed(() => {
  if (resizingTop.value) return x.elementY.value;
  if (waitingForTopUpdate.value) {
    return waitingForTopUpdate.value;
  }
  return props.availability.normalized.from;
});

const computedBottom = computed(() => {
  if (resizingBottom.value) return x.elementY.value;
  if (waitingForBottomUpdate.value) {
    return waitingForBottomUpdate.value;
  }
  return props.availability.normalized.to;
});

const handleUpdate = async () => {
  await weeklyAvailabilityStore.handleUpdate({
    id: props.availability.model.id,
    params: {
      normalized: {
        from: computedTop.value,
        to: computedBottom.value,
      },
      weekday: props.weekday,
    },
  });

  waitingForTopUpdate.value = null;
  waitingForBottomUpdate.value = null;
};

watch(resizingTop, async (resizingTop) => {
  weeklyAvailabilityStore.isUpdating = resizingTop;
  if (!resizingTop) {
    waitingForTopUpdate.value = x.elementY.value;
    await nextTick();
    handleUpdate();
  }
});

watch(resizingBottom, async (resizingBottom) => {
  weeklyAvailabilityStore.isUpdating = resizingBottom;
  if (!resizingBottom) {
    waitingForBottomUpdate.value = x.elementY.value;
    await nextTick();
    handleUpdate();
  }
});
</script>

<template>
  <div
    :style="{
      height: `${computedBottom - computedTop}px`,
      top: `${computedTop}px`,
      left: '0',
    }"
    :class="computedBottom - computedTop > 100 ? 'flex-col' : 'flex-row'"
    class="bg-neutral-200 absolute w-full h-full flex flex-col justify-between items-center p-2 rounded"
  >
    <div ref="topResizer" class="absolute top-0 left-1/2 -translate-1/2">
      <UButton
        ref="topResizer"
        size="xs"
        icon="lucide:chevron-up"
        class="cursor-n-resize h-4"
      />
    </div>
    <p class="relative z-10 text-black font-strong">
      {{ getAvailabilityTime(computedTop) }}
    </p>

    <UButton
      v-if="!waitingForBottomUpdate || !waitingForBottomUpdate"
      class="cursor-pointer"
      :size="computedBottom - computedTop > 100 ? 'md' : 'xs'"
      icon="lucide:trash-2"
      @click="weeklyAvailabilityStore.handleRemove(availability.model.id)"
    />
    <UButton
      v-else
      disabled
      loading
      :size="computedBottom - computedTop > 100 ? 'md' : 'xs'"
    />
    <p class="relative z-10 text-black font-strong">
      {{ getAvailabilityTime(computedBottom) }}
    </p>

    <div
      ref="bottomResizer"
      class="absolute bottom-0 left-1/2 -translate-x-1/2 translate-y-1/2 cursor-s-resize z-0 h-4"
    >
      <UButton
        size="xs"
        icon="lucide:chevron-down"
        class="cursor-s-resize h-4"
      />
    </div>
  </div>
</template>
