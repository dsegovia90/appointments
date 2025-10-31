import { useStorage } from "@vueuse/core";

interface ApiOptions<B> {
  method?: string;
  body?: B;
  urlSearchParams?: URLSearchParams;
}

export async function api<T, B = void>(
  path: string,
  options?: ApiOptions<B>,
): Promise<T> {
  const token = useStorage("token", "");

  const fetchOptions = {
    method: options?.method || "GET",
    body: JSON.stringify(options?.body),
    headers: {
      Authorization: `Bearer ${token.value}`,
      "Content-Type": "application/json",
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
    },
  };

  const response = await fetch(
    path +
      (options?.urlSearchParams
        ? `?${options.urlSearchParams.toString()}`
        : ""),
    fetchOptions,
  );

  if (!response.ok) {
    throw new Error(response.statusText);
  }

  return (await response.json()) as T;
}
