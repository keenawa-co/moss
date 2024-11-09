import { useContext } from "react";

import { InstantiationContext } from "@/lib/instantiation/instantiationContext";
import { ServiceIdentifier } from "@/lib/instantiation/serviceCollection";

/**
 * Custom hook to retrieve a service from the InstantiationService.
 * @param serviceId - The ServiceIdentifier of the desired service.
 * @returns The instance of the requested service.
 */
export function useService<T>(serviceId: ServiceIdentifier<T>): T {
  const instantiationService = useContext(InstantiationContext);

  if (!instantiationService) {
    throw new Error("InstantiationService is not available in the context. Make sure to initialize the app correctly.");
  }

  return instantiationService.invokeFunction((accessor) => accessor.get(serviceId));
}
