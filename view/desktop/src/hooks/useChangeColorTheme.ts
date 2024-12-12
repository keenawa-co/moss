import { invokeMossCommand } from "@/lib/backend/platfrom";
import { ThemeDescriptor } from "@repo/desktop-models";
import { useMutation, useQueryClient } from "@tanstack/react-query";

const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
  await invokeMossCommand("workbench.changeColorTheme", {
    themeDescriptor,
  });
};

export const useChangeTheme = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, ThemeDescriptor>({
    mutationKey: ["changeTheme"],
    mutationFn: changeTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["storedString"] });
    },
  });
};
