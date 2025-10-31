import { api } from "~/utils/api";
import type { CreateUpdateParams } from "~/bindings/CreateUpdateParams";
import type { WeeklyAvailabilityByWeekday } from "~/bindings/WeeklyAvailabilityByWeekday";
import type { WeeklyAvailability } from "~/bindings/WeeklyAvailability";

export const useWeeklyAvailabilityStore = defineStore(
  "weeklyAvailability",
  () => {
    const availabilityWeek = ref<WeeklyAvailabilityByWeekday>();
    const toast = useToast();
    const isUpdating = ref(false);

    const handleCreate = async (params: CreateUpdateParams) => {
      await api<WeeklyAvailability, CreateUpdateParams>(
        `/api/weekly_availabilities`,
        {
          method: "POST",
          body: params,
        },
      );

      await fetchAvailability();

      toast.add({
        title: "Time window created!",
        icon: "i-lucide-calendar-days",
      });
    };

    const fetchAvailability = async () => {
      availabilityWeek.value = await api<WeeklyAvailabilityByWeekday>(
        "/api/weekly_availabilities",
      );
    };

    onMounted(fetchAvailability);

    const handleUpdate = async ({
      id,
      params,
    }: {
      id: number;
      params: CreateUpdateParams;
    }) => {
      await api<WeeklyAvailability, CreateUpdateParams>(
        `/api/weekly_availabilities/${id}`,
        {
          method: "PUT",
          body: params,
        },
      );

      await fetchAvailability();

      toast.add({
        title: "Time window updated!",
        icon: "i-lucide-calendar-days",
      });
    };

    const handleRemove = async (availabilityId: number) => {
      // Implement the logic to remove the availability
      availabilityWeek.value = await api<WeeklyAvailabilityByWeekday>(
        `/api/weekly_availabilities/${availabilityId}`,
        {
          method: "DELETE",
        },
      );

      toast.add({
        title: "Time window deleted!",
        icon: "i-lucide-calendar-days",
      });
    };

    return {
      availabilityWeek,
      handleCreate,
      handleRemove,
      handleUpdate,
      isUpdating,
    };
  },
);
