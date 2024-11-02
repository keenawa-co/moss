import { injectable, inject } from "inversify";
import { container } from "../../di/container";
import { Event } from "../eventBus";
import { Channels, EventsOf } from "../eventTypes";
import { TYPES } from "@/di/types";

@injectable()
export class EventFilter {
  constructor() {}

  async applyFilters<Channel extends Channels, EventName extends EventsOf<Channel>>(
    event: Event<Channel, EventName>
  ): Promise<boolean> {
    return true;
  }
}

export async function applyFilters<Channel extends Channels, EventName extends EventsOf<Channel>>(
  event: Event<Channel, EventName>
): Promise<boolean> {
  const filter = container.get<EventFilter>(TYPES.EventFilter);
  return filter.applyFilters(event);
}
