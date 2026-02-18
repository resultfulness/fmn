use fmn_backend::run;
use tokio::task::{self, JoinHandle};

pub struct State {
    task: JoinHandle<()>,
}

pub async fn setup() -> State {
    let t = task::spawn(async {
        run().await;
    });
    State { task: t }
}

pub async fn teardown(state: State) {
    state.task.abort();
}
