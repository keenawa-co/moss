import { useMutation, useQueryClient } from "@tanstack/react-query";

const changeTheme = async (_newString: string): Promise<void> => {
  // FIXME: replace this when we have the Appearance object on the backend.
  // await invoke("set_stored_string", { newString });
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
