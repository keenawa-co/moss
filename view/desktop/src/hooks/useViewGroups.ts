import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface ViewGroup {
  id: string;
  title: string;
  order: number;
  icon: string;
  active: boolean;
}

// ViewGroups
let ViewGroups: ViewGroup[] = [
  {
    "id": "explorer",
    "title": "Explorer",
    "order": 1,
    "icon": "ActivityBarIcon1",
    active: true,
  },
  {
    "id": "activities",
    "title": "Activities",
    "order": 2,
    "icon": "ActivityBarIcon2",
    active: false,
  },
];

export const getViewGroups = async () => {
  await new Promise((resolve) => setTimeout(resolve, 50));
  return ViewGroups;
};

export const useGetViewGroups = () => {
  return useQuery<ViewGroup[], Error>({
    queryKey: ["getViewGroups"],
    queryFn: getViewGroups,
  });
};

export const useChangeViewGroups = () => {
  const queryClient = useQueryClient();

  return useMutation<ViewGroup[], Error, ViewGroup[]>({
    mutationFn: async (newViewGroups) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      ViewGroups = newViewGroups;
      return newViewGroups;
    },
    onSuccess() {
      queryClient.invalidateQueries({ queryKey: ["getViewGroups"] });
    },
  });
};

// ViewGroup

interface GroupView {
  id: string;
  name: string;
}

export const getViewGroup = async (groupId: string) => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  if (groupId === "explorer") {
    return {
      "id": "explorer",
      "name": "My View1",
    };
  }
  if (groupId === "activities") {
    return {
      "id": "activities",
      "name": "My View2",
    };
  }

  return null;
};

export const useGetViewGroup = (groupId: string) => {
  return useQuery<GroupView | null, Error>({
    queryKey: ["getViewGroup", groupId],
    queryFn: () => getViewGroup(groupId),
    enabled: !!groupId,
  });
};
