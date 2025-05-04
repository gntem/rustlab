use std::sync::{Arc, Mutex};

use tokio::sync::mpsc;

enum Action {
    Increment,
    Decrement,
    BlockedAction
}

struct Store {
    state: i32,
}

impl Store {
    fn new() -> Self {
        Store { state: 0 }
    }

    fn dispatch(&mut self, action: Action) {
        match action {
            Action::Increment => self.state += 1,
            Action::Decrement => self.state -= 1,
            Action::BlockedAction => {
                println!("Blocked action received");
            }
        }
    }

    fn get_state(&self) -> i32 {
        self.state
    }
}

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(Store::new()));
    let (tx, mut rx) = mpsc::channel::<Action>(32);
    let store_clone = Arc::clone(&store);

    tokio::spawn(async move {
        while let Some(action) = rx.recv().await {
            let mut s = store_clone.lock().unwrap();
            s.dispatch(action);
        }
    });

    tx.send(Action::Increment).await.unwrap();
    tx.send(Action::Increment).await.unwrap();
    tx.send(Action::Increment).await.unwrap();
    tx.send(Action::BlockedAction).await.unwrap();
    tx.send(Action::Increment).await.unwrap();
    tx.send(Action::Increment).await.unwrap();
    tx.send(Action::BlockedAction).await.unwrap();
    tx.send(Action::Decrement).await.unwrap();

    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let s = store.lock().unwrap();
    println!("{}", s.get_state());
}