import { createFetch, useStorage } from "@vueuse/core";

export const useAPI = createFetch({
  options: {
    async beforeFetch({ options }) {
      const token = useStorage("token", "");

      options.headers = {
        ...options.headers,
        Authorization: `Bearer ${token.value}`,
        "Content-Type": "application/json",
        timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
      };

      if (options.body) {
        options.body = JSON.stringify(options.body);
      }

      return { options };
    },
  },
  fetchOptions: {
    mode: "cors",
  },
});
