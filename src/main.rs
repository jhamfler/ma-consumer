use env_logger;
use failure::Error;
use futures::{future::Future, Stream};
use lapin_futures as lapin;
use crate::lapin::client::ConnectionOptions;
use crate::lapin::channel::{BasicConsumeOptions, QueueDeclareOptions};
use crate::lapin::types::FieldTable;
use log::{debug, info};
use tokio;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use std::env;

fn main() {
  let args: Vec<_> = env::args().collect();
  if args.len() < 2 {
    println!("arg needed: ip:port");
    return;
  }

  env_logger::init();

  //let addr = "192.168.99.100:31525".parse().unwrap();
  let addr = args[1].parse().unwrap();

  Runtime::new().unwrap().block_on_all(
    TcpStream::connect(&addr).map_err(Error::from).and_then(|stream| {

      // connect() returns a future of an AMQP Client
      // that resolves once the handshake is done
      lapin::client::Client::connect(stream, ConnectionOptions::default()).map_err(Error::from)
   }).and_then(|(client, heartbeat)| {
     // The heartbeat future should be run in a dedicated thread so that nothing can prevent it from
     // dispatching events on time.
     // If we ran it as part of the "main" chain of futures, we might end up not sending
     // some heartbeats if we don't poll often enough (because of some blocking task or such).
     println!("connect");
     tokio::spawn(heartbeat.map_err(|_| ()));
     println!("heartbeat");

      // create_channel returns a future that is resolved
      // once the channel is successfully created
      client.create_channel().map_err(Error::from)
    }).and_then(|channel| {
      println!("create_channel");
      let id = channel.id;
      info!("created channel with id: {}", id);
      println!("id is: {}", id);

      let ch = channel.clone();
      channel.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::new()).and_then(move |queue| {
        info!("channel {} declared queue {}", id, "hello");
        println!("channel {} declared queue {}", id, "hello");

        // basic_consume returns a future of a message
        // stream. Any time a message arrives for this consumer,
        // the for_each method would be called
        channel.basic_consume(&queue, "my_consumer", BasicConsumeOptions::default(), FieldTable::new())
      }).and_then(|stream| {
        info!("got consumer stream");
        println!("got consumer stream");

        stream.for_each(move |message| {
          debug!("got message: {:?}", message);
          info!("decoded message: {:?}", std::str::from_utf8(&message.data).unwrap());
          println!("decoded message: {:?}", std::str::from_utf8(&message.data).unwrap());
          ch.basic_ack(message.delivery_tag, false)
        })
      }).map_err(Error::from)
    })
  ).expect("runtime failure");
}
