import { injectable, inject } from "inversify";
import { Subject, Subscription } from "rxjs";
import PQueue from "p-queue";
import { filter, mergeMap } from "rxjs/operators";
import { EventPayloads, Channels, EventsOf, PayloadOf } from "./eventTypes";
import { Logger } from "./logger";
import { getPriority } from "./priorities";
import { loadHandler } from "./handlerLoader";
import { TYPES } from "@/di/types";

export interface Event<Channel extends Channels, EventName extends EventsOf<Channel>> {
  channel: Channel;
  event: EventName;
  payload: PayloadOf<Channel, EventName>;
}

@injectable()
export class EventBus {
  private eventSubjects = new Map<Channels, Subject<Event<any, any>>>();
  private subscriptions = new Map<Channels, Subscription>();
  private queue = new PQueue({ concurrency: 1, autoStart: true });
  private logger: Logger;

  constructor(@inject(TYPES.Logger) logger: Logger) {
    this.logger = logger;
  }

  subscribeChannel(channel: Channels) {
    if (this.eventSubjects.has(channel)) {
      // Already subscribed
      return;
    }

    const subject = new Subject<Event<any, any>>();
    this.eventSubjects.set(channel, subject);

    const subscription = subject
      .pipe(
        // All events pass through (no filters)
        mergeMap(async (event) => {
          const priority = getPriority(event.channel, event.event);
          const handler = await loadHandler(event.channel, event.event);
          return { handler, event, priority };
        })
      )
      .subscribe(({ handler, event, priority }) => {
        this.queue.add(
          () =>
            handler(event.payload).catch((err) => {
              this.logger.error(`Error processing event ${event.channel}:${event.event}`, err);
            }),
          { priority }
        );
      });

    this.subscriptions.set(channel, subscription);
  }

  unsubscribeChannel(channel: Channels) {
    if (this.subscriptions.has(channel)) {
      this.subscriptions.get(channel)!.unsubscribe();
      this.subscriptions.delete(channel);
      this.eventSubjects.delete(channel);
    }
  }

  private getEventSubject(channel: Channels): Subject<Event<any, any>> | undefined {
    return this.eventSubjects.get(channel);
  }

  getSubscribedChannels(): Channels[] {
    return Array.from(this.eventSubjects.keys());
  }

  receiveEvent<Channel extends Channels, EventName extends EventsOf<Channel>>(
    channel: Channel,
    event: EventName,
    payload: PayloadOf<Channel, EventName>
  ) {
    const subject = this.getEventSubject(channel);
    if (subject) {
      subject.next({ channel, event, payload });
    } else {
      this.logger.log(`Received event for unsubscribed channel: ${channel}`);
    }
  }
}
