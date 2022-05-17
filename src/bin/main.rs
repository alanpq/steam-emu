use std::net::TcpListener;

use tracing::{info, metadata::LevelFilter};

extern crate libloading;
use libloading::{Library, Symbol};

fn main() -> color_eyre::Result<()> {
  color_eyre::install()?;
  tracing_subscriber::fmt()
    .with_max_level(LevelFilter::DEBUG)
    .init();

  let listener = TcpListener::bind("127.0.0.1:1337")?;
  info!("TCP Socket open.");

  unsafe {
    let lib = Library::new("./spacewar/steam_api.dll")?;

  }


  let (mut stream, addr) = listener.accept()?;
  info!(%addr, "Accepted");

  let mut stdout = std::io::stdout();

  std::io::copy(&mut stream, &mut stdout).unwrap();


  Ok(())
}