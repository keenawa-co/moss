import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

const changeTheme = async (newString: string): Promise<void> => {
  await invoke("set_stored_string", { newString });
};

export const useChangeTheme = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationKey: ["changeTheme"],
    mutationFn: changeTheme,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["storedString"] });
    },
  });
};
