import { useQuery } from "@tanstack/react-query";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";

const fetchStoredString = async (): Promise<string> => {
  const result = await invoke<string>("get_stored_string");
  return result;
};

export const useStoredString = () => {
  return useQuery<string, Error>({
    queryKey: ["storedString"],
    // staleTime: 1000 * 60 * 5,
    queryFn: fetchStoredString,
    // refetchOnWindowFocus: false,
  });
};

const updateStoredString = async (newString: string): Promise<void> => {
  await invoke("set_stored_string", { newString });
};

export const useUpdateStoredString = () => {
  const queryClient = useQueryClient();

  return useMutation<void, Error, string>({
    mutationKey: ["updateStoredString"],
    mutationFn: updateStoredString,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["storedString"] });
    },
  });
};
