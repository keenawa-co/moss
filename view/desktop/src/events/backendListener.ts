import { listen } from "@tauri-apps/api/event";
import { container } from "../di/container";
import { TYPES } from "../di/types";
import { EventBus } from "./eventBus";
import { Logger } from "./logger";
import { Channels } from "./eventTypes";
import { eventNames } from "./eventNames";

export async function initializeBackendListeners(channels: Channels[]) {
  const eventBus = container.get<EventBus>(TYPES.EventBus);
  const logger = container.get<Logger>(TYPES.Logger);

  const listenerPromises = channels.map(async (channel) => {
    const events = eventNames[channel];
    await Promise.all(
      events.map(async (eventName) => {
        try {
          await listen<any>(`${channel}:${eventName}`, (event) => {
            logger.log(`Received event ${eventName} from channel ${channel}`);

            // Pass the event to EventBus
            eventBus.receiveEvent(channel, eventName, event.payload);
          });
          logger.log(`Listener set up for ${channel}:${eventName}`);
        } catch (err) {
          logger.error(`Error listening to event ${eventName} on channel ${channel}`, err);
        }
      })
    );
  });

  await Promise.all(listenerPromises);
}
