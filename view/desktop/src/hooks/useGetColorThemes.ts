import { ThemeDescriptor } from "@repo/desktop-models";
import { useQuery } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

const getColorThemes = async (): Promise<ThemeDescriptor[]> => {
  return await invoke<ThemeDescriptor[]>("get_themes");
};

export const useGetColorThemes = () => {
  return useQuery<ThemeDescriptor[], Error>({
    queryKey: ["getColorThemes"],
    queryFn: getColorThemes,
  });
};
