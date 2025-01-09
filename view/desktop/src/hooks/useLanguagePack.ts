import { getLanguagePacks } from "@/api/appearance";
import { invokeMossCommand } from "@/lib/backend/platfrom";
import { LocaleDescriptor } from "@repo/moss-desktop";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export const useGetLanguagePacks = () => {
  return useQuery<LocaleDescriptor[], Error>({
    queryKey: ["getLanguagePacks"],
    queryFn: getLanguagePacks,
  });
};

const changeLanguagePack = async (localeDescriptor: LocaleDescriptor): Promise<void> => {
  await invokeMossCommand("workbench.changeLanguagePack", {
    localeDescriptor,
  });
};

export const useChangeLanguagePack = () => {
  const queryClient = useQueryClient();
  return useMutation<void, Error, LocaleDescriptor>({
    mutationKey: ["changeLanguagePack"],
    mutationFn: changeLanguagePack,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    },
  });
};
