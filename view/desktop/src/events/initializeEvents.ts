import { initializeBackendListeners } from "./backendListener";
import { Channels } from "./eventTypes";
import { container } from "../di/container";
import { TYPES } from "../di/types";
import { EventBus } from "./eventBus";

export async function initializeEvents() {
  const eventBus = container.get<EventBus>(TYPES.EventBus);

  // Channels to subscribe to immediately
  const initialChannels: Channels[] = ["channel1"];

  await initializeBackendListeners(initialChannels);

  initialChannels.forEach((channel) => {
    eventBus.subscribeChannel(channel);
  });
}
