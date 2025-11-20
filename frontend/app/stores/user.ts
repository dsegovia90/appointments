import type { MagicLinkParams } from "~/bindings/MagicLinkParams";
import type { LoginParams } from "~/bindings/LoginParams";
import type { LoginResponse } from "~/bindings/LoginResponse";
import type { RegisterParams } from "~/bindings/RegisterParams";
import { api } from "~/utils/api";
import { defineStore } from "pinia";
import type { ResetParams } from "~/bindings/ResetParams";
import type { ForgotParams } from "~/bindings/ForgotParams";
import { useStorage } from "@vueuse/core";

type User = LoginResponse;

export const useUserStore = defineStore("user", () => {
  const route = useRoute();
  const router = useRouter();
  const user = ref<User | null>(null);
  const loading = ref(false);
  const toast = useToast();
  const token = useStorage<string>("token", "");

  onMounted(() => {
    fetchCurrentUser();
  });

  const fetchCurrentUser = async () => {
    await api<LoginResponse>("/api/auth/current")
      .then((response) => {
        user.value = response;
      })
      .catch((error) => {
        setUser(null);
        console.error(error);
      })
      .finally(() => {
        loading.value = true;
      });
  };

  const setUser = (newUser: User | null) => {
    user.value = newUser;
    token.value = newUser?.token || "";
  };

  const handleRegister = async (registerParams: RegisterParams) => {
    await api<LoginResponse, RegisterParams>("/api/auth/register", {
      method: "POST",
      body: registerParams,
    })
      .then(setUser)
      .catch((error) => {
        toast.add({
          title: "Error sending registering!",
          icon: "lucide:mail-x",
          description: "Please try again later.",
        });
        console.error(error);
      });
  };

  const handleLogin = async (loginParams: LoginParams) => {
    await api<LoginResponse, LoginParams>("/api/auth/login", {
      method: "POST",
      body: loginParams,
    })
      .then(setUser)
      .catch((error) => {
        console.error(error);
      });
  };

  const handleMagicLink = async (
    magicLinkParams: MagicLinkParams,
    sideEffect: () => void,
  ) => {
    await api<unknown, MagicLinkParams>("/api/auth/magic-link", {
      method: "POST",
      body: magicLinkParams,
    })
      .then(() => {
        toast.add({
          title: "Magic link sent!",
          icon: "lucide:mail",
        });
      })
      .catch((error) => {
        toast.add({
          title: "Error sending magic link!",
          icon: "lucide:mail-x",
        });
        console.error(error);
      })
      .finally(sideEffect);
  };

  const verifyMagicLink = async (token: string) => {
    await api<LoginResponse, MagicLinkParams>(`/api/auth/magic-link/${token}`)
      .then(setUser)
      .catch((error) => {
        toast.add({
          title: "Error verifying magic link!",
          description: "Please try again.",
          icon: "lucide:mail-x",
        });
        router.push("/");
        console.error(error);
      });
  };

  const handleLogout = async () => {
    await api("/api/auth/logout").catch((error) => {
      console.error(error);
    });
    setUser(null);
  };

  const handlePasswordReset = async (params: ForgotParams) => {
    await api<unknown, ForgotParams>("/api/auth/forgot", {
      method: "POST",
      body: params,
    })
      .then(() => {
        toast.add({
          title: "Password reset email sent!",
          icon: "lucide:mail",
        });
      })
      .catch((error) => {
        toast.add({
          title: "Error sending password reset email!",
          icon: "lucide:mail-x",
        });
        console.error(error);
      });
  };

  const handlePasswordChange = async (params: ResetParams) => {
    await api<unknown, ResetParams>("/api/auth/reset", {
      method: "POST",
      body: params,
    })
      .then(() => {
        toast.add({
          title: "Password changed!",
          icon: "lucide:check",
        });
      })
      .catch((error) => {
        toast.add({
          title: "Error changing password!",
          icon: "lucide:x",
        });
        console.error(error);
      });
  };

  watch([user, loading], ([user, loading]) => {
    if (!loading) {
      return;
    }

    if (!user && route.path.startsWith("/dashboard")) {
      router.push("/login");
    } else if (user && !route.path.startsWith("/dashboard")) {
      router.push("/dashboard");
    }
  });

  return {
    user,
    loading,
    fetchCurrentUser,
    handleLogin,
    handleMagicLink,
    handleLogout,
    handleRegister,
    handlePasswordReset,
    handlePasswordChange,
    verifyMagicLink,
  };
});
