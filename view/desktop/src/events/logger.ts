import { injectable } from "inversify";

@injectable()
export class Logger {
  log(message: string) {
    console.log(`[INFO]: ${message}`);
  }

  error(message: string, error?: any) {
    console.error(`[ERROR]: ${message}`, error);
  }
}
