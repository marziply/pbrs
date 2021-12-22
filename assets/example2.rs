#[derive(Clone)]
pub struct GreeterClient {
  client: ::grpcio::Client
}

impl GreeterClient {
  pub fn new(channel: ::grpcio::Channel) -> Self {
    GreeterClient {
      client: ::grpcio::Client::new(channel)
    }
  }

  pub fn say_hello_opt(
    &self,
    req: &super::message::HelloRequest,
    opt: ::grpcio::CallOption
  ) -> ::grpcio::Result<super::message::HelloReply> {
    self
      .client
      .unary_call(&METHOD_GREETER_SAY_HELLO, req, opt)
  }

  pub fn say_hello(
    &self,
    req: &super::message::HelloRequest
  ) -> ::grpcio::Result<super::message::HelloReply> {
    self.say_hello_opt(req, ::grpcio::CallOption::default())
  }
}

pub trait Greeter {
  fn say_hello(
    &mut self,
    ctx: ::grpcio::RpcContext,
    _req: super::message::HelloRequest,
    sink: ::grpcio::UnarySink<super::message::HelloReply>
  ) {
    grpcio::unimplemented_call!(ctx, sink)
  }
}

pub fn create_greeter<S: Greeter + Send + Clone + 'static>(
  s: S
) -> ::grpcio::Service {
  let mut builder = ::grpcio::ServiceBuilder::new();
  let mut instance = s;
  builder = builder
    .add_unary_handler(&METHOD_GREETER_SAY_HELLO, move |ctx, req, resp| {
      instance.say_hello(ctx, req, resp)
    });
  builder.build()
}
