import { EventBus, IEventBus } from "../events/eventBus";
import { Channels } from "../events/eventTypes";
import { ILoggerService } from "./loggerService";
import { createDecorator } from "../lib/instantiation/instantiation";
import { eventNames } from "@/events/eventNames";
import { listen } from "@tauri-apps/api/event";

export interface IEventService {
  initialize(channels: Channels[]): Promise<void>;
  getEventBus(): IEventBus;
}

export const IEventService = createDecorator<IEventService>("eventService");

export class EventService implements IEventService {
  private eventBus: IEventBus;

  constructor(@ILoggerService private logger: ILoggerService) {
    this.eventBus = new EventBus(this.logger);
  }

  async initialize(channels: Channels[]): Promise<void> {
    const listenerPromises = channels.map(async (channel) => {
      const events = eventNames[channel];

      events.map(async (eventName) => {
        try {
          await listen<any>(`${channel}:${eventName}`, (event) => {
            this.logger.log(`Received event ${eventName} from channel ${channel}`);

            this.eventBus.receiveEvent(channel, eventName, event.payload);
          });
          this.logger.log(`Listener set up for ${channel}:${eventName}`);
        } catch (err) {
          this.logger.error(`Error listening to event ${eventName} on channel ${channel}`, err);
        }
      });
    });

    await Promise.all(listenerPromises);

    channels.forEach((channel) => {
      this.eventBus.subscribeChannel(channel);
    });
  }

  getEventBus(): IEventBus {
    return this.eventBus;
  }
}
