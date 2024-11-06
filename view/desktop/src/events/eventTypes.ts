// Define Channels
export type Channels = "channel1";

// Define Events for each Channel
export type EventsOfChannel1 = "eventA";

// Map Events to Channels
export type EventsOf<Channel extends Channels> = Channel extends "channel1" ? EventsOfChannel1 : never;

// Define Payloads for each Event
export interface EventPayloads {
  channel1: {
    eventA: { data: string };
  };
}

// Get Payload Type for a Given Channel and Event
export type PayloadOf<
  Channel extends Channels,
  EventName extends EventsOf<Channel>,
> = EventPayloads[Channel][EventName];
