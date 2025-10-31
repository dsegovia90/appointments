import type { AppointmentType } from "~/bindings/AppointmentType";
import type { CreateAppointmentTypeParams } from "~/bindings/CreateAppointmentTypeParams";
import type { UpdateAppointmentTypeParams } from "~/bindings/UpdateAppointmentTypeParams";
import { api } from "~/utils/api";

export const useAppointmentTypesAPI = () => {
  const toast = useToast();

  const fetchAll = async (): Promise<AppointmentType[]> => {
    try {
      return await api<AppointmentType[]>("/api/appointment_types");
    } catch (error) {
      console.log(error);
      toast.add({
        title: "Error fetching appointment types",
        description: "Failed to load appointment types. Please try again.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  const fetchOne = async (id: number): Promise<AppointmentType> => {
    try {
      return await api<AppointmentType>(`/api/appointment_types/${id}`);
    } catch (error) {
      toast.add({
        title: "Error fetching appointment type",
        description: "Failed to load appointment type details.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  const create = async (
    params: CreateAppointmentTypeParams,
  ): Promise<AppointmentType> => {
    try {
      const result = await api<AppointmentType, CreateAppointmentTypeParams>(
        "/api/appointment_types",
        {
          method: "POST",
          body: params,
        },
      );
      toast.add({
        title: "Success",
        description: "Appointment type created successfully",
        icon: "i-lucide-check-circle",
      });
      return result;
    } catch (error) {
      toast.add({
        title: "Error creating appointment type",
        description: "Failed to create appointment type. Please try again.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  const update = async (
    id: number,
    params: UpdateAppointmentTypeParams,
  ): Promise<AppointmentType> => {
    try {
      const result = await api<AppointmentType, UpdateAppointmentTypeParams>(
        `/api/appointment_types/${id}`,
        {
          method: "PUT",
          body: params,
        },
      );
      toast.add({
        title: "Success",
        description: "Appointment type updated successfully",
        icon: "i-lucide-check-circle",
      });
      return result;
    } catch (error) {
      toast.add({
        title: "Error updating appointment type",
        description: "Failed to update appointment type. Please try again.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  const destroy = async (id: number): Promise<void> => {
    try {
      await api(`/api/appointment_types/${id}`, {
        method: "DELETE",
      });
      toast.add({
        title: "Success",
        description: "Appointment type deleted successfully",
        icon: "i-lucide-check-circle",
      });
    } catch (error) {
      toast.add({
        title: "Error deleting appointment type",
        description: "Failed to delete appointment type. Please try again.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  return {
    fetchAll,
    fetchOne,
    create,
    update,
    destroy,
  };
};
