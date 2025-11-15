// frontend/app/stores/appointmentTypesAdmin.ts
import type { AppointmentType } from "~/bindings/AppointmentType";
import type { CreateAppointmentTypeParams } from "~/bindings/CreateAppointmentTypeParams";
import type { UpdateAppointmentTypeParams } from "~/bindings/UpdateAppointmentTypeParams";

export const useAppointmentTypesAdminStore = defineStore(
  "appointmentTypesAdmin",
  () => {
    const api = useAppointmentTypesAPI();

    const appointmentTypesMap = ref<Map<number, AppointmentType>>(new Map());
    const loading = ref(true);
    const selectedAppointmentType = ref<AppointmentType | null>(null);

    const fetchAppointmentTypes = async () => {
      loading.value = true;
      try {
        const fetchedAppointmentTypes = await api.fetchAll();
        appointmentTypesMap.value = new Map(
          fetchedAppointmentTypes.map((type) => [type.id, type]),
        );
      } catch (error) {
        console.error("Failed to fetch appointment types:", error);
      } finally {
        loading.value = false;
      }
    };

    const createAppointmentType = async (
      params: CreateAppointmentTypeParams,
    ) => {
      loading.value = true;
      try {
        const newAppointmentType = await api.create(params);
        appointmentTypesMap.value.set(
          newAppointmentType.id,
          newAppointmentType,
        );
        return newAppointmentType;
      } catch (error) {
        console.error("Failed to create appointment type:", error);
        throw error;
      } finally {
        loading.value = false;
      }
    };

    const updateAppointmentType = async (
      id: number,
      params: UpdateAppointmentTypeParams,
    ) => {
      loading.value = true;
      try {
        const updatedAppointmentType = await api.update(id, params);
        appointmentTypesMap.value.set(
          updatedAppointmentType.id,
          updatedAppointmentType,
        );
        return updatedAppointmentType;
      } catch (error) {
        console.error("Failed to update appointment type:", error);
        throw error;
      } finally {
        loading.value = false;
      }
    };

    const deleteAppointmentType = async (id: number) => {
      loading.value = true;
      try {
        await api.destroy(id);
        appointmentTypesMap.value.delete(id);
      } catch (error) {
        console.error("Failed to delete appointment type:", error);
        throw error;
      } finally {
        loading.value = false;
      }
    };

    const selectAppointmentType = (appointmentType: AppointmentType | null) => {
      selectedAppointmentType.value = appointmentType;
    };

    onMounted(() => {
      fetchAppointmentTypes();
    });

    return {
      appointmentTypesMap,
      loading,
      selectedAppointmentType,
      fetchAppointmentTypes,
      createAppointmentType,
      updateAppointmentType,
      deleteAppointmentType,
      selectAppointmentType,
    };
  },
);
