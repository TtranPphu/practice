use std::{
  io::Write,
  sync::{Arc, Mutex},
};

use chrono::Local;
use fern::Dispatch;
use log::{
  debug, error, info, trace, warn,
  LevelFilter::{Debug, Info, Trace, Warn},
};

pub fn init() {
  let console_logger = Dispatch::new().level(Debug).chain(std::io::stderr());

  let file_logger = Dispatch::new()
    .level(Info)
    .chain(fern::log_file("log.log").unwrap());

  let papertrail_connection = Arc::new(Mutex::new(
    std::net::TcpStream::connect("logs6.papertrailapp.com:11443").unwrap(),
  ));

  let papertrail_logger = Dispatch::new()
    .level(Warn)
    .chain(fern::Output::call(move |record| {
      let log_message = format!(
        "<22>{} {}: {}",
        Local::now().format("%Y-%m-%dT%H:%M:%S"),
        record.target(),
        record.args()
      );
      let mut connection = papertrail_connection.lock().unwrap();
      connection.write_all(log_message.as_bytes()).unwrap();
    }));

  let root_logger = Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{} [{}] [{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        record.target(),
        record.level(),
        message
      ))
    })
    .level(Trace)
    .chain(console_logger)
    .chain(file_logger)
    .chain(papertrail_logger);

  match root_logger.apply() {
    Ok(_) => {
      println!("Logger initialized");
    }
    Err(e) => {
      eprintln!("Failed to initialize logger: {}", e);
    }
  }
}

pub fn test_log() {
  init();
  info!("info");
  warn!("warn");
  error!("error");
  trace!("trace");
  debug!("debug");
}
