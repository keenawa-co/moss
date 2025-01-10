import { getColorThemes } from "@/api/appearance";
import { invokeMossCommand } from "@/lib/backend/platfrom";
import { ThemeDescriptor } from "@repo/moss-desktop";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetColorThemes = () => {
  return useQuery<ThemeDescriptor[], Error>({
    queryKey: ["getColorTheme"],
    queryFn: getColorThemes,
  });
};

const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
  await invokeMossCommand("workbench.changeColorTheme", {
    themeDescriptor,
  });
};

export const useChangeColorTheme = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, ThemeDescriptor>({
    mutationKey: ["changeColorTheme"],
    mutationFn: changeTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};
