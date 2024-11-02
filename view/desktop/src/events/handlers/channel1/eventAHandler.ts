import { injectable, inject } from "inversify";
import { TYPES } from "../../../di/types";
import { Logger } from "../../logger";
import { PayloadOf } from "../../eventTypes";
import { container } from "../../../di/container";

@injectable()
export class EventAHandler {
  constructor(@inject(TYPES.Logger) private logger: Logger) {}

  async handle(payload: PayloadOf<"channel1", "eventA">): Promise<void> {
    this.logger.log(`Handling eventA: ${payload.data}`);
    // Some logic here
  }
}

export const createEventAHandler = (): EventAHandler => {
  return container.get<EventAHandler>(EventAHandler);
};
