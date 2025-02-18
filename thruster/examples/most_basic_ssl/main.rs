#![feature(async_await, proc_macro_hygiene)]
extern crate thruster;

use thruster::{MiddlewareNext, MiddlewareReturnValue};
use thruster::{App, BasicContext as Ctx, Request};
use thruster::ssl_server::SSLServer;
use thruster::ThrusterServer;
use thruster::thruster_proc::{async_middleware, middleware_fn};

#[middleware_fn]
async fn plaintext(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
  let val = "Hello, World!";
  context.body(val);
  context
}

#[middleware_fn]
async fn test_fn_404(mut context: Ctx, _next: MiddlewareNext<Ctx>) -> Ctx {
  context.body("404");
  context
}

fn main() {
  println!("Starting server...");

  let mut app = App::<Request, Ctx>::new_basic();

  app.get("/plaintext", async_middleware!(Ctx, [plaintext]));
  app.set404(async_middleware!(Ctx, [test_fn_404]));

  let mut server = SSLServer::new(app);
  server.cert(include_bytes!("identity.p12").to_vec());
  server.cert_pass("asdfasdfasdf");
  server.start("0.0.0.0", 4321);
}
