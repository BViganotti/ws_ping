Compile the project with: "wasm-pack build --target web"

Testing with http-server using the index.html:
    Run: "npx http-server" to serve index.html
    Go to [http://127.0.0.1:8080](http://127.0.0.1:8080) and watch the console where http-server is running to see the traffic

Testing with Node.js:
    If you have already compiled the project, make sure to change whithin the pkg/package.json file the value from "type": "module" to "type": "commonjs" otherwise Nodejs won't be able to require the function properly
    Make sure you have wasm-bindgen-cli installed, if you don't run: "cargo install wasm-bindgen-cli"
    Run: "wasm-bindgen target/wasm32-unknown-unknown/release/ws_ping.wasm --out-dir ./pkg --nodejs" to make it Nodejs compatible
    In the nodejs_ws_ping_test directory run: "npm install"
    Finally to run the test (assuming yo're still in the nodejs_ws_ping_test dir) run: "npm test"
    The output should look like this:
    ❯ npm test

    > nodejs_ws_ping_test@1.0.0 test
    > node test.js

    received: Hello, WebSocket!
    Response from WebSocket server: Echo: Hello, WebSocket!