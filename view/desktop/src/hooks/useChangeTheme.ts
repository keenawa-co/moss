import { invokeIpc } from "@/lib/backend/tauri";
import { ThemeDescriptor } from "@repo/desktop-models";
import { useMutation, useQueryClient } from "@tanstack/react-query";

const changeTheme = async (themeDescriptor: ThemeDescriptor): Promise<void> => {
  // FIXME: replace this when we have the Appearance object on the backend.
  // await invoke("handle_signal", { newString });

  await invokeIpc<unknown, string>("set_color_theme", {
    themeDescriptor: themeDescriptor,
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
