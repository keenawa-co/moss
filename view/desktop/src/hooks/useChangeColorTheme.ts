import { invokeMossCommand } from "@/lib/backend/platfrom";
import { ThemeDescriptor } from "@repo/moss-desktop";
import { useMutation, useQueryClient } from "@tanstack/react-query";

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
