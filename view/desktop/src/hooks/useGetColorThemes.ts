import { getColorThemes } from "@/api/appearance";
import { ThemeDescriptor } from "@repo/moss-desktop";
import { useQuery } from "@tanstack/react-query";

export const useGetColorThemes = () => {
  return useQuery<ThemeDescriptor[], Error>({
    queryKey: ["getColorTheme"],
    queryFn: getColorThemes,
  });
};
