import { ILoggerService } from "@/services/loggerService";
import { PayloadOf } from "../../eventTypes";

export interface IEventAHandler {
  handle(payload: PayloadOf<"channel1", "eventA">): Promise<void>;
}

export class EventAHandler implements IEventAHandler {
  private logger: ILoggerService;

  constructor(logger: ILoggerService) {
    this.logger = logger;
  }

  async handle(payload: PayloadOf<"channel1", "eventA">): Promise<void> {
    this.logger.log(`Handling eventA: ${payload.data}`);
    // Some logic here
  }
}
