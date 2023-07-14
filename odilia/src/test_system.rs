use actix::prelude::*;

use crate::exit_handler::StopSystem;

pub struct HelloSystem;

impl Actor for HelloSystem {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		println!("Hello system started.");
	}
}

impl Handler<StopSystem> for HelloSystem {
	type Result = ();

	fn handle(&mut self, msg: StopSystem, ctx: &mut Self::Context) -> Self::Result {
		println!("Stop signal received.");
		ctx.stop();
	}
}
