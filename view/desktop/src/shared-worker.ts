export {};

declare const self: SharedWorkerGlobalScope;

self.onconnect = (event) => {
  const port = event.ports[0];

  // Initialize your service here
  let serviceInitialized = false;
  const service = {
    // Your service implementation
    initialize: () => {
      if (!serviceInitialized) {
        // Perform initialization logic
        console.log("Service initialized");
        serviceInitialized = true;
      }
    },
    // Other service methods
    doSomething: () => {
      // Your method implementation
      return "Result from service";
    },
  };

  // Initialize the service once
  service.initialize();

  port.onmessage = (msgEvent) => {
    const { action, data } = msgEvent.data;

    switch (action) {
      case "callMethod":
        // Call a method on the service and post the result back
        // const result = service[data.method](...data.args);
        console.log("-------");
        port.postMessage({ action: "result", data: "Hello, World!" });
        break;

      // Handle other actions
      default:
        console.warn("Unknown action:", action);
    }
  };
};
