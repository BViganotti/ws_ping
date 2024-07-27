const wasm = require('../pkg/ws_ping');

(async () => {
  try {
    // create a ws server to test against
    const WebSocket = require('ws');
    const server = new WebSocket.Server({ port: 8080 });

    server.on('connection', ws => {
      ws.on('message', message => {
        console.log('received: %s', message);
        ws.send(`Echo: ${message}`);
      });
    });

    // test the ws_ping function
    const response = await wasm.ws_ping('ws://localhost:8080', 'Hello, WebSocket!');
    console.log('Response from WebSocket server:', response);

    // close the server
    server.close();
  } catch (e) {
    console.error(e);
  }
})();