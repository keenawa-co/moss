import { Channels, EventsOf } from "./eventTypes";

export const eventNames: {
  [Channel in Channels]: EventsOf<Channel>[];
} = {
  channel1: ["eventA"],
};
