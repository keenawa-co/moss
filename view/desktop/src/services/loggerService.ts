import { createDecorator } from "../lib/instantiation/instantiation";

export interface ILoggerService {
  log(message: string): void;
  error(message: string, error?: any): void;
}

export const ILoggerService = createDecorator<ILoggerService>("loggerService");

export class LoggerService implements ILoggerService {
  log(message: string): void {
    console.log(`[INFO]: ${message}`);
  }

  error(message: string, error?: any): void {
    console.error(`[ERROR]: ${message}`, error);
  }
}
