import { useEffect, useState } from "react";
import { useAppDispatch } from "../store";
import { setLanguageFromLocalStorage } from "../store/languages/languagesSlice";
import { initializeThemes } from "../store/themes";
import { mainWindowIsReadyCommand } from "../tauri";
import { ILoggerService, LoggerService } from "@/services/loggerService";
import { ServiceCollection } from "@/lib/instantiation/serviceCollection";
import { InstantiationService } from "@/lib/instantiation/instantiationService";
import { SyncDescriptor } from "@/lib/instantiation/descriptor";
import { EventService, IEventService } from "@/services/eventService";
import { IInstantiationService } from "@/lib/instantiation/instantiation";
import { Channels } from "@/events/eventTypes";

export const useInitializeApp = () => {
  const dispatch = useAppDispatch();

  const [isInitializing, setIsInitializing] = useState(true);
  const [initializationError, setInitializationError] = useState<Error | null>(null);
  const [instantiationService, setInstantiationService] = useState<IInstantiationService | null>(null);

  useEffect(() => {
    let isDisposed = false;

    const initialize = async () => {
      try {
        const services = new ServiceCollection();

        services.set(ILoggerService, new SyncDescriptor(LoggerService));
        services.set(IEventService, new SyncDescriptor(EventService));

        const instantiationService = new InstantiationService(services);
        setInstantiationService(instantiationService);

        const eventService = instantiationService.invokeFunction((accessor) => accessor.get(IEventService));

        // Initialize EventService
        const initialChannels: Channels[] = ["channel1"];
        await eventService.initialize(initialChannels);

        mainWindowIsReadyCommand();

        // Dispatch Redux actions
        dispatch(setLanguageFromLocalStorage());
        dispatch(initializeThemes());
      } catch (error) {
        console.error("Initialization error:", error);
        if (!isDisposed) {
          setInitializationError(error as Error);
        }
      } finally {
        if (!isDisposed) {
          setIsInitializing(false);
        }
      }
    };

    initialize();

    return () => {
      isDisposed = true;
      // Dispose of the InstantiationService and its services
      if (instantiationService) {
        instantiationService.dispose();
      }
    };
  }, [dispatch]);

  return { isInitializing, initializationError, instantiationService };
};
