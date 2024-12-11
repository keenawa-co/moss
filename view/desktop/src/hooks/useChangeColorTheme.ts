import { invokeIpc } from "@/lib/backend/tauri";
import { ThemeDescriptor } from "@repo/desktop-models";
import { useMutation, useQueryClient } from "@tanstack/react-query";

const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
  await invokeIpc<unknown, string>("execute_command", {
    commandId: "workbench.changeColorTheme",
    args: {
      themeDescriptor: themeDescriptor,
    },
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
