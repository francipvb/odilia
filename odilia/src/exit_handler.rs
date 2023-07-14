use actix::{fut::wrap_future, prelude::*};

pub struct ExitHandlerActor {
	subscriptions: Vec<Recipient<StopSystem>>,
}

impl ExitHandlerActor {
	pub fn new() -> Self {
		Self { subscriptions: Vec::new() }
	}
}

#[derive(Message, Clone)]
#[rtype("()")]
pub enum StopSystem {
	Gracefully,
}

impl Actor for ExitHandlerActor {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		let addr = ctx.address();
		ctx.spawn(wrap_future(async move {
			let _ = tokio::signal::ctrl_c().await;
			let _ = addr.send(StopSystem::Gracefully).await;
		}));
	}

	fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
		if self.subscriptions.is_empty() {
			Running::Stop
		} else {
			Running::Continue
		}
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		System::current().stop();
	}
}

impl Handler<StopSystem> for ExitHandlerActor {
	type Result = ();

	fn handle<'a>(&'a mut self, msg: StopSystem, ctx: &mut Self::Context) -> Self::Result {
		let subscriptions = self.subscriptions.clone();
		let addr = ctx.address();
		ctx.spawn(wrap_future(async move {
			for item in subscriptions.clone().iter() {
				if item.connected() {
					let _ = item.send(msg.clone()).await;
				}
				let _ = addr.send(Unsubscribe(item.clone())).await;
			}
		}));
		ctx.stop()
	}
}

#[derive(Message)]
#[rtype("()")]
pub struct Subscribe {
	subscriber: Recipient<StopSystem>,
}

impl Subscribe {
	pub fn new(subscriber: Recipient<StopSystem>) -> Self {
		Self { subscriber }
	}
}

impl Handler<Subscribe> for ExitHandlerActor {
	type Result = ();

	fn handle(&mut self, msg: Subscribe, _ctx: &mut Self::Context) -> Self::Result {
		self.subscriptions.push(msg.subscriber.clone().to_owned());
	}
}

#[derive(Message)]
#[rtype("bool")]
struct Unsubscribe(Recipient<StopSystem>);

impl Handler<Unsubscribe> for ExitHandlerActor {
	type Result = bool;

	fn handle(&mut self, msg: Unsubscribe, ctx: &mut Self::Context) -> Self::Result {
		for (index, item) in self.subscriptions.clone().iter().enumerate().rev() {
			if *item == msg.0 {
				self.subscriptions.remove(index);
				return true;
			}
		}
		false
	}
}
