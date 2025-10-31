import type { AppointmentType } from "@/bindings/AppointmentType";

export const useAppointmentTypeStore = defineStore("appointmentType", () => {
  const appointmentTypes = ref<Map<number, AppointmentType>>(new Map());
  const loading = ref<boolean>(true);

  onMounted(async () => {
    const response = await api<AppointmentType[]>(
      "/api/client-facing/appointment-types/1",
    );
    appointmentTypes.value = new Map(response.map((item) => [item.id, item]));
    loading.value = false;
  });

  return { appointmentTypes, loading };
});
