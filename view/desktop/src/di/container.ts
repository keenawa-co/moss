import { Container } from "inversify";
import { TYPES } from "./types";
import { EventBus } from "../events/eventBus";
import { Logger } from "@/events/logger";
import { EventAHandler } from "@/events/handlers/channel1/eventAHandler";
import { EventFilter } from "@/events/filters/eventFilter";

const container = new Container();

// container.bind<EventBus>(TYPES.EventBus).to(EventBus).inSingletonScope();
// container.bind<EventFilter>(TYPES.EventFilter).to(EventFilter).inSingletonScope();

// container.bind<Logger>(TYPES.Logger).to(Logger).inSingletonScope();

// container.bind<EventAHandler>(EventAHandler).toSelf();

container.bind<EventBus>(TYPES.EventBus).to(EventBus).inSingletonScope();
container.bind<Logger>(TYPES.Logger).to(Logger).inSingletonScope();
container.bind<EventAHandler>(TYPES.EventAHandler).to(EventAHandler);

export { container };
