import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

export interface ProjectSessionState {
  lastActiveGroup: string;
  changedViews: ChangedView[];
}

interface ChangedView {
  id: string;
  collapsed: boolean;
}

let projectSessionState = {
  "lastActiveGroup": "explorer.groupId",
  changedViews: [
    {
      "id": "explorer.groupId",
      collapsed: false,
    },
    {
      "id": "activities.groupId",
      collapsed: false,
    },
  ],
};

export const useGetProjectSessionState = () => {
  return useQuery<ProjectSessionState, Error>({
    queryKey: ["getProjectState"],
    queryFn: async () => {
      await new Promise((resolve) => setTimeout(resolve, 50));
      return projectSessionState;
    },
  });
};

export const useChangeProjectSessionState = () => {
  const queryClient = useQueryClient();

  return useMutation<ProjectSessionState, Error, ProjectSessionState>({
    mutationFn: async (newProjectState) => {
      await new Promise((resolve) => setTimeout(resolve, 50));

      projectSessionState = newProjectState;

      return newProjectState;
    },
    onSuccess(newProjectState) {
      queryClient.setQueryData(["getProjectState"], newProjectState);
    },
  });
};
