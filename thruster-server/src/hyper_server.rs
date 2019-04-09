use std::net::ToSocketAddrs;
use std::collections::HashMap;

use futures_legacy::Future;
use tokio;
use hyper::{Body, Response, Request, Server as HyperServer};
use hyper::service::service_fn;
use std::sync::Arc;

use thruster_app::app::App;
use crate::thruster_server::ThrusterServer;
use thruster_core::context::Context;
use thruster_context::basic_hyper_context::HyperRequest;

pub struct Server<T: 'static + Context + Send> {
  app: App<HyperRequest, T>
}

impl<T: 'static + Context + Send> Server<T> {
}

impl<T: Context<Response = Response<Body>> + Send> ThrusterServer for Server<T> {
  type Context = T;
  type Response = Response<Body>;
  type Request = HyperRequest;

  fn new(app: App<Self::Request, T>) -> Self {
    Server {
      app
    }
  }

  fn start(mut self, host: &str, port: u16) {
    let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();

    self.app._route_parser.optimize();
    let arc_app = Arc::new(self.app);

    let new_service = move || {
      let clone = arc_app.clone();
      service_fn(move |req: Request<Body>| {
        let matched = clone.resolve_from_method_and_path(
          &req.method().to_string(),
          &req.uri().to_string()
        );
        clone.resolve(HyperRequest(req), matched)
      })
    };

    let server = HyperServer::bind(&addr)
      .serve(new_service)
      .map_err(|e| eprintln!("server error: {}", e));

    tokio::run(server);
  }
}
