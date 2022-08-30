use std::{
	collections::HashMap,
	sync::{atomic::AtomicUsize, Arc},
};

use anyhow::Result;
use futures_util::{FutureExt, StreamExt};
use tokio::{
	runtime::Builder,
	sync::{mpsc, RwLock},
};
use warp::{ws::Message, Filter};

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

fn main() -> Result<()> {
	let rt = Builder::new_current_thread().enable_all().build()?;

	rt.block_on(run())?;

	Ok(())
}

async fn run() -> Result<()> {
	let users = Users::default();

	let users = warp::any().map(move || users.clone());

	let chat = warp::path("chat")
		.and(warp::ws())
		.and(users)
		.map(|ws: warp::ws::Ws, users| todo!());

	Ok(())
}
