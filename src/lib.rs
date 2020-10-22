use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    stream::Stream,
};

pub use futures::stream::StreamExt;

#[cfg(target_arch = "wasm32")]
mod rt {
    use super::*;
    use wasm_bindgen::{prelude::*, JsCast};
    use web_sys::{MessageEvent, Worker};

    pub(crate) fn spawn<T>(script_url: &str) -> Result<(Sender<&[u8]>, Receiver<T>), ()>
    where
        T: for<'de> serde::Deserialize<'de> + 'static,
    {
        let w = Worker::new(script_url).map_err(|_| ())?;

        let (mut msg_tx, rx) = channel(0);
        let (tx, _msg_rx) = channel(0);

        let on_msg = Closure::wrap(Box::new(move |e: MessageEvent| {
            let data = e.data().into_serde().unwrap();
            msg_tx.try_send(data).unwrap();
        }) as Box<dyn FnMut(MessageEvent)>);

        w.set_onmessage(Some(on_msg.as_ref().unchecked_ref()));
        on_msg.forget();

        Ok((tx, rx))
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod rt {
    use super::*;
    //use deno_core::{error::AnyError, JsRuntime};

    pub(crate) fn spawn<T>(_script_url: &str) -> Result<(Sender<&[u8]>, Receiver<T>), ()>
    where
        T: for<'de> serde::Deserialize<'de> + 'static,
    {
        todo!()
    }
}

pub struct Worker<T> {
    messages: Receiver<T>,
}

impl<T> Worker<T>
where
    T: for<'de> serde::Deserialize<'de> + 'static,
{
    pub fn new(script_url: &str) -> Self {
        let (_tx, rx) = rt::spawn(script_url).unwrap();
        Worker { messages: rx }
    }

    pub fn messages(&mut self) -> impl Stream<Item = T> + '_ {
        self.messages.by_ref()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        // let _w = Worker::new("");
        assert!(true);
    }
}
