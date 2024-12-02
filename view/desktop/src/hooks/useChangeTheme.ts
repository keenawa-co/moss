import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { ThemeDescriptor } from "@repo/desktop-models";

const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
  await invoke("set_selected_theme", { selectedTheme: themeDescriptor });
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
