# WebAssembly WebSocket Ping

## Compile the Project

Compile the project with:

```sh
wasm-pack build --target web
```

Testing with HTTP Server

To test using http-server with index.html:

	1.	Run the following command to serve index.html:
    ```sh
    npx http-server
    ```
    2.	Go to http://127.0.0.1:8080 and watch the console where http-server is running to see the traffic.

Testing with Node.js

	1.	If you have already compiled the project, ensure the pkg/package.json file has "type": "commonjs" instead of "type": "module". Otherwise, Node.js won’t be able to require the function properly.
	2.	Ensure you have wasm-bindgen-cli installed. If you don’t, run:
    ```sh
    cargo install wasm-bindgen-cli
    ```
    3.	Run the following command to make it Node.js compatible:
    ```sh
    wasm-bindgen target/wasm32-unknown-unknown/release/ws_ping.wasm --out-dir ./pkg --nodejs
    ```
    4.	In the nodejs_ws_ping_test directory, install the dependencies:
    ```sh
    npm install
    ```
    5.	Finally, to run the test (assuming you’re still in the nodejs_ws_ping_test directory), run:
    ```sh
    npm test
    ```
    
    Expected Output
    ```sh
    ❯ npm test

    > nodejs_ws_ping_test@1.0.0 test
    > node test.js
    
    received: Hello, WebSocket!
    Response from WebSocket server: Echo: Hello, WebSocket!
    ```
