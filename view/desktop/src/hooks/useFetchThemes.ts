import { ThemeDescriptor } from "@/api/theme";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

const fetchThemes = async (): Promise<ThemeDescriptor[]> => {
  const result = await invoke<ThemeDescriptor[]>("fetch_themes");
  return result;
};

export const useFetchThemes = () => {
  return useQuery<ThemeDescriptor[], Error>({
    queryKey: ["fetchThemes"],
    queryFn: fetchThemes,
  });
};
