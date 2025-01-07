import { getState } from "@/api/appearance";
import { AppState } from "@repo/moss-desktop";
import { useQuery } from "@tanstack/react-query";

export const useGetAppState = () => {
  return useQuery<AppState, Error>({
    queryKey: ["getState"],
    queryFn: getState,
  });
};
