import { getLanguagePacks } from "@/api/appearance";
import { LocaleDescriptor } from "@repo/moss-desktop";
import { useQuery } from "@tanstack/react-query";

export const useGetLanguagePacks = () => {
  return useQuery<LocaleDescriptor[], Error>({
    queryKey: ["getLanguagePacks"],
    queryFn: getLanguagePacks,
  });
};
