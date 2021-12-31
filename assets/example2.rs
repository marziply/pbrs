#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use super::message::{HelloReply, HelloRequest};
use grpcio::{
  pb_de as de,
  pb_ser as ser,
  CallOption,
  Channel,
  Client,
  Marshaller,
  Method,
  MethodType,
  RpcContext,
  ServiceBuilder,
  UnarySink
};

const GREETER_SAY_HELLO: Method<HelloRequest, HelloReply> = Method {
  ty: MethodType::Unary,
  name: "/pbrs.Greeter/SayHello",
  req_mar: Marshaller {
    ser,
    de
  },
  resp_mar: Marshaller {
    ser,
    de
  }
};

#[derive(Clone)]
pub struct GreeterClient {
  client: Client
}

impl GreeterClient {
  pub fn new(channel: Channel) -> Self {
    Self {
      client: Client::new(channel)
    }
  }

  pub fn say_hello_opt(
    &self,
    req: &HelloRequest,
    opt: CallOption
  ) -> Result<HelloReply> {
    self
      .client
      .unary_call(&GREETER_SAY_HELLO, req, opt)
  }

  pub fn say_hello(&self, req: &HelloRequest) -> Result<HelloReply> {
    self.say_hello_opt(req, CallOption::default())
  }
}

pub trait Greeter {
  fn say_hello(
    &mut self,
    ctx: RpcContext,
    req: HelloRequest,
    sink: UnarySink<HelloReply>
  ) {
    unimplemented_call!(ctx, sink)
  }
}

pub fn create_greeter<S: Greeter + Send + Clone + 'static>(s: S) -> Service {
  ServiceBuilder::new()
    .add_unary_handler(&GREETER_SAY_HELLO, s.say_hello)
    .build()
}
