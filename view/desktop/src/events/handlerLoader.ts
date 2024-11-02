import { Channels, EventsOf, PayloadOf } from "./eventTypes";
import { EventAHandler } from "./handlers/channel1/eventAHandler";
import { container } from "../di/container";
import { TYPES } from "../di/types";

export async function loadHandler(
  channel: "channel1",
  event: "eventA"
): Promise<(payload: PayloadOf<"channel1", "eventA">) => Promise<void>>;

export async function loadHandler<Channel extends Channels, EventName extends EventsOf<Channel>>(
  channel: Channel,
  event: EventName
): Promise<(payload: PayloadOf<Channel, EventName>) => Promise<void>>;

export async function loadHandler(channel: Channels, event: string): Promise<(payload: any) => Promise<void>> {
  switch (channel) {
    case "channel1":
      switch (event) {
        case "eventA": {
          const handlerInstance = container.get<EventAHandler>(TYPES.EventAHandler);
          return handlerInstance.handle.bind(handlerInstance);
        }
        default:
          throw new Error(`Handler not found for event ${event} in channel ${channel}`);
      }
    default:
      throw new Error(`Handler not found for channel ${channel}`);
  }
}
