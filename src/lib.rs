use futures::channel::oneshot;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

#[wasm_bindgen]
pub async fn ws_ping(endpoint: String, message: String) -> Result<JsValue, JsValue> {
    let ws: WebSocket = WebSocket::new(&endpoint).map_err(|err| JsValue::from(err))?;
    // to signal when the WebSocket is open
    let (send_tx, send_rx) = oneshot::channel::<()>();
    // to receive the response message
    let (recv_tx, recv_rx) = oneshot::channel::<String>();
    // wrap it to share it among multiple closures
    let send_tx = Rc::new(RefCell::new(Some(send_tx)));

    let on_open = {
        // clone the Rc to share ownership
        let send_tx = Rc::clone(&send_tx);
        Closure::wrap(Box::new(move || {
            // take the send_tx out of the RefCell, sending a signal through the channel
            if let Some(tx) = send_tx.borrow_mut().take() {
                tx.send(()).unwrap();
            }
        }) as Box<dyn FnMut()>)
    };
    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();

    // wait for the signal that the WebSocket is open
    let _ = send_rx.await;

    // send the message
    ws.send_with_str(&message)
        .map_err(|err| JsValue::from(err))?;

    // closure for the onmessage event of the WebSocket
    let recv_tx = Rc::new(RefCell::new(Some(recv_tx)));
    let on_message = {
        let recv_tx = Rc::clone(&recv_tx);
        Closure::wrap(Box::new(move |event: MessageEvent| {
            // extract the text message from the event data
            if let Some(text) = event.data().as_string() {
                if let Some(tx) = recv_tx.borrow_mut().take() {
                    // send the received message through the recv_tx channel
                    tx.send(text).unwrap();
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>)
    };
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();
    // waiting for the response message from the WebSocket
    let response = recv_rx.await.unwrap();

    Ok(JsValue::from_str(&response))
}
