#![deny(
	clippy::all,
	clippy::pedantic,
	clippy::cargo,
	clippy::map_unwrap_or,
	clippy::unwrap_used,
	unsafe_code
)]
#![allow(clippy::multiple_crate_versions)]

mod events;
mod exit_handler;
mod logging;
mod state;
mod test_system;

use actix::prelude::*;
use exit_handler::{ExitHandlerActor, Subscribe};
use test_system::HelloSystem;


fn main() -> Result<(), std::io::Error> {
	let system = System::new();
	system.block_on(async {
		register_systems().await;
	});
	system.run()
}

async fn register_systems() {
	let exit_handler = ExitHandlerActor::new().start();
	exit_handler
		.send(Subscribe::new(HelloSystem.start().recipient()))
		.await;
}
