use std::net::TcpListener;

use tracing::{info, metadata::LevelFilter, error};

extern crate libloading;
use libloading::{Library, Symbol};

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

  let listener = TcpListener::bind("127.0.0.1:1337")?;
  info!("TCP Socket open.");

  let mut stdout = std::io::stdout();


  loop {
    let (mut stream, addr) = listener.accept()?;
    info!(%addr, "Accepted");


    match std::io::copy(&mut stream, &mut stdout) {
      Ok(_) => {
        info!("TCP Socket closed.")
      },
      Err(err) => {
        error!(%err);
      },
    };
  }


  Ok(())
}