import { Channels, EventsOf } from "./eventTypes";

type Priority = number; // Lower number means higher priority

const priorityMap: {
  [Channel in Channels]: {
    [Event in EventsOf<Channel>]: Priority;
  };
} = {
  channel1: {
    eventA: 1,
  },
};

export function getPriority<Channel extends Channels, EventName extends EventsOf<Channel>>(
  channel: Channel,
  event: EventName
): Priority {
  return priorityMap[channel]?.[event] ?? 999; // Default low priority
}
