use irc::client::prelude::Client;
use irc::client::prelude::*;
use futures::prelude::*;
use irc::client::prelude::*;
use std::default::Default;
use std::env;
extern crate irc;


fn main() {
	irc_client_system();
}


async fn irc_client_system() {
	let mut client = Client::new("config.toml").await.expect("wahh");
	let mut stream = client.stream().expect("stream broke");
	while let Some(message) = stream.next().await.transpose().expect("loop broke") {
		let content = message.clone().command;
		println!("{}",message);
	}

}
