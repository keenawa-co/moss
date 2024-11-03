import { ILoggerService } from "@/services/loggerService";
import { Channels, EventsOf, PayloadOf } from "./eventTypes";
import { EventAHandler } from "./handlers/channel1/eventAHandler";

export async function loadHandler(
  channel: "channel1",
  event: "eventA",
  logger: ILoggerService
): Promise<(payload: PayloadOf<"channel1", "eventA">) => Promise<void>>;

export async function loadHandler<Channel extends Channels, EventName extends EventsOf<Channel>>(
  channel: Channel,
  event: EventName,
  logger: ILoggerService
): Promise<(payload: PayloadOf<Channel, EventName>) => Promise<void>>;

export async function loadHandler(
  channel: Channels,
  event: string,
  logger: ILoggerService
): Promise<(payload: any) => Promise<void>> {
  switch (channel) {
    case "channel1":
      switch (event) {
        case "eventA": {
          const handlerInstance = new EventAHandler(logger);
          return handlerInstance.handle.bind(handlerInstance);
        }
        default:
          throw new Error(`Handler not found for event ${event} in channel ${channel}`);
      }
    default:
      throw new Error(`Handler not found for channel ${channel}`);
  }
}
