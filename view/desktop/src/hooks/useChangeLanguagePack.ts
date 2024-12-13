import { invokeIpc } from "@/lib/backend/tauri.ts";
import { useMutation } from "@tanstack/react-query";
import { LanguagePack } from "@/store/language";

const changeLanguagePack = async (languagePack: LanguagePack): Promise<void> => {
  await invokeIpc<unknown, string>("execute_command", {
    commandId: "workbench.changeLanguagePack",
    args: {
      code: languagePack.code,
    },
  });
};

export const useChangeLanguagePack = () => {
  // const queryClient = useQueryClient();

  return useMutation<void, Error, LanguagePack>({
    mutationKey: ["changeLanguagePack"],
    mutationFn: changeLanguagePack,
    onSuccess: () => {},
  });
};
