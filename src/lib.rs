use futures::channel::oneshot;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

#[wasm_bindgen]
pub async fn ws_ping(endpoint: String, message: String) -> Result<JsValue, JsValue> {
    let ws: WebSocket = WebSocket::new(&endpoint).map_err(|err| JsValue::from(err))?;

    let (send_tx, send_rx) = oneshot::channel::<()>();
    let (recv_tx, recv_rx) = oneshot::channel::<String>();

    let send_tx = Rc::new(RefCell::new(Some(send_tx)));

    let on_open = {
        let send_tx = Rc::clone(&send_tx);
        Closure::wrap(Box::new(move || {
            if let Some(tx) = send_tx.borrow_mut().take() {
                tx.send(()).unwrap();
            }
        }) as Box<dyn FnMut()>)
    };
    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();

    let _ = send_rx.await;

    ws.send_with_str(&message)
        .map_err(|err| JsValue::from(err))?;

    let recv_tx = Rc::new(RefCell::new(Some(recv_tx)));
    let on_message = {
        let recv_tx = Rc::clone(&recv_tx);
        Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                if let Some(tx) = recv_tx.borrow_mut().take() {
                    tx.send(text).unwrap();
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>)
    };
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();

    let response = recv_rx.await.unwrap();

    Ok(JsValue::from_str(&response))
}
