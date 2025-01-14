import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface ViewGroup {
  id: string;
  title: string;
  order: number;
  icon: string;
  active: boolean;
}

export interface Views {
  viewGroups: ViewGroup[];
}

// Views
let Views: Views = {
  "viewGroups": [
    {
      "id": "explorer.groupId",
      "title": "Explorer",
      "order": 1,
      "icon": "ActivityBarIcon1",
      active: true,
    },
    {
      "id": "activities.groupId",
      "title": "Activities",
      "order": 2,
      "icon": "ActivityBarIcon2",
      active: false,
    },
  ],
};

export const useGetViewGroups = () => {
  return useQuery<Views, Error>({
    queryKey: ["getViewGroups"],
    queryFn: async () => {
      await new Promise((resolve) => setTimeout(resolve, 50));
      return Views;
    },
  });
};

export const useChangeViewGroups = () => {
  const queryClient = useQueryClient();

  return useMutation<Views, Error, Views>({
    mutationFn: async (newViewGroups) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      Views = newViewGroups;

      return newViewGroups;
    },
    onSuccess(newViewGroups) {
      console.log("onSuccess");
      // queryClient.invalidateQueries({ queryKey: ["getViewGroups"] });
      queryClient.setQueryData(["getViewGroups"], newViewGroups);
    },
  });
};

// ViewGroup

interface GroupView {
  id: string;
  name: string;
  component: string;
}

const getViewGroup = async (groupId: string) => {
  await new Promise((resolve) => setTimeout(resolve, 50));

  if (groupId === "explorer.groupId") {
    return {
      "id": "explorer",
      "name": "My View1",
      "component": "AccordionsList",
    };
  }
  if (groupId === "activities.groupId") {
    return {
      "id": "activities",
      "name": "My View2",
      "component": "ActivitiesList",
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
