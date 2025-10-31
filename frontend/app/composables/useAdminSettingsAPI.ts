import type { AdminSettings } from "~/bindings/AdminSettings";
import type { AdminSettingsParams } from "~/bindings/AdminSettingsParams";
import { api } from "~/utils/api";

export const useAdminSettingsAPI = () => {
  const toast = useToast();

  const fetch = async (): Promise<AdminSettings> => {
    try {
      return await api<AdminSettings>("/api/admin_settings");
    } catch (error) {
      toast.add({
        title: "Error fetching admin settings.",
        description: "Failed to load admin settings. Please try again.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  const update = async (
    params: Partial<AdminSettingsParams>,
  ): Promise<AdminSettings> => {
    try {
      const result = await api<AdminSettings, Partial<AdminSettingsParams>>(
        `/api/admin_settings`,
        {
          method: "PUT",
          body: params,
        },
      );
      toast.add({
        title: "Success",
        description: "Admin settings updated successfully",
        icon: "i-lucide-check-circle",
      });
      return result;
    } catch (error) {
      toast.add({
        title: "Error updating admin settings",
        description: "Failed to update admin settings. Please try again.",
        icon: "i-lucide-alert-circle",
      });
      throw error;
    }
  };

  return {
    fetch,
    update,
  };
};
