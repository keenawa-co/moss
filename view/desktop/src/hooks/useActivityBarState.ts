import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface ActivityBarState {
  position: "top" | "bottom" | "left" | "right";
}

let ActivityBarState = {
  position: "top",
};

const getActivityBarState = async () => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return ActivityBarState as ActivityBarState;
};

export const useGetActivityBarState = () => {
  return useQuery<ActivityBarState, Error>({
    queryKey: ["getActivityBar"],
    queryFn: getActivityBarState,
  });
};

export const useChangeActivityBarState = () => {
  const queryClient = useQueryClient();
  return useMutation<ActivityBarState, Error, ActivityBarState>({
    mutationFn: async (newLayout) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      ActivityBarState = newLayout;

      return newLayout;
    },
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: ["getActivityBar"] });
    },
  });
};
