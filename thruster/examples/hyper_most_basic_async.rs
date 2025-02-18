#![feature(async_await, proc_macro_hygiene)]
extern crate thruster;

use thruster::{MiddlewareNext, MiddlewareReturnValue};
use thruster::{App, ThrusterServer};
use thruster::hyper_server::{HyperServer};
use thruster::thruster_context::basic_hyper_context::{generate_context, BasicHyperContext as Ctx, HyperRequest};
use thruster::thruster_proc::{async_middleware, middleware_fn};
use hyper::Body;

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
  let val = "Hello, World!";
  context.body = Body::from(val);
  context
}

fn main() {
  println!("Starting server...");

  let mut app = App::<HyperRequest, Ctx>::create(generate_context);
  app.get("/plaintext", async_middleware!(Ctx, [plaintext]));

  let server = HyperServer::new(app);
  server.start("0.0.0.0", 4321);
}
