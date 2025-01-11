import { getAllActivities } from "@/api/appearance";
import { MenuItem } from "@repo/moss-desktop";
import { useQuery } from "@tanstack/react-query";

export const useGetActivitiesState = () => {
  return useQuery<MenuItem[], Error>({
    queryKey: ["getAllActivities"],
    queryFn: async () => {
      const result = await getAllActivities();
      if (result.status === "ok") {
        return result.data;
      } else if (result.status === "error") {
        throw result.error;
      }
      throw new Error("Unexpected response status");
    },
  });
};
