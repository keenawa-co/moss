import { invokeMossCommand } from "@/lib/backend/platfrom";
import { LocaleDescriptor } from "@repo/moss-desktop";
import { useMutation } from "@tanstack/react-query";

const changeLanguagePack = async (localeDescriptor: LocaleDescriptor): Promise<void> => {
  await invokeMossCommand("workbench.changeLanguagePack", {
    localeDescriptor,
  });
};

export const useChangeLanguagePack = () => {
  // const queryClient = useQueryClient();

  return useMutation<void, Error, LocaleDescriptor>({
    mutationKey: ["changeLanguagePack"],
    mutationFn: changeLanguagePack,
    onSuccess: () => {},
  });
};
