import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface ActivityBarState {
  position: "top" | "bottom" | "left" | "right";
}

let ActivityBarState = {
  position: "top",
};

export const useGetActivityBarState = () => {
  return useQuery<ActivityBarState, Error>({
    queryKey: ["getActivityBar"],
    queryFn: async () => {
      await new Promise((resolve) => setTimeout(resolve, 50));
      return ActivityBarState as ActivityBarState;
    },
  });
};

export const useChangeActivityBarState = () => {
  const queryClient = useQueryClient();
  return useMutation<ActivityBarState, Error, ActivityBarState>({
    mutationFn: async (newLayout) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      queryClient.invalidateQueries({ queryKey: ["getActivityBar"] });

      ActivityBarState = newLayout;
      return newLayout;
    },
  });
};
