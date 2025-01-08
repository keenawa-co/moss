import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export type LayoutAlignment = "center" | "justify" | "left" | "right";
export type LayoutPrimarySideBarPosition = "left" | "right";
export interface AppLayoutState {
  alignment: LayoutAlignment;
  primarySideBarPosition: "left" | "right";
  primarySideBar: {
    width: number;
    visibility: boolean;
  };
  secondarySideBar: {
    width: number;
    visibility: boolean;
  };
  bottomPane: {
    height: number;
    visibility: boolean;
  };
}

let AppLayout: AppLayoutState = {
  alignment: "center",
  primarySideBarPosition: "left",
  primarySideBar: {
    width: 255,
    visibility: true,
  },
  secondarySideBar: {
    width: 255,
    visibility: true,
  },
  bottomPane: {
    height: 333,
    visibility: true,
  },
};

export const useGetAppLayoutState = () => {
  return useQuery<AppLayoutState, Error>({
    queryKey: ["getLayout"],
    queryFn: async () => {
      await new Promise((resolve) => setTimeout(resolve, 50));
      return AppLayout;
    },
  });
};
export const useChangeAppLayoutState = () => {
  const queryClient = useQueryClient();
  return useMutation<AppLayoutState, Error, AppLayoutState>({
    mutationFn: async (newLayout) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      queryClient.invalidateQueries({ queryKey: ["getLayout"] });

      AppLayout = newLayout;
      return newLayout;
    },
  });
};
