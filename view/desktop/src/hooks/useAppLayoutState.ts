import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface AppLayoutState {
  alignment: "center" | "justify" | "left" | "right";
  primarySideBarPosition: "left" | "right";
}

let AppLayoutState = {
  alignment: "center",
  primarySideBarPosition: "left",
};

const getAppLayoutState = async () => {
  await new Promise((resolve) => setTimeout(resolve, 0));
  return AppLayoutState as AppLayoutState;
};

export const useGetAppLayoutState = () => {
  return useQuery<AppLayoutState, Error>({
    queryKey: ["getAppLayoutState"],
    queryFn: getAppLayoutState,
  });
};

export const useChangeAppLayoutState = () => {
  const queryClient = useQueryClient();

  return useMutation<AppLayoutState, Error, AppLayoutState>({
    mutationFn: async (newLayout) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      AppLayoutState = newLayout;
      return newLayout;
    },
    onSuccess(data, variables, context) {
      console.log("onSuccess", { data, variables, context });
      queryClient.invalidateQueries({ queryKey: ["getAppLayoutState"] });
    },
  });
};
