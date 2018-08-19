use clap;

use tokio;
use tokio::io;
use tokio::io::AsyncRead;
use tokio::net::TcpListener;

use tokio_process::{Child, CommandExt};

use futures::{Future, Stream};

use std::process::{Command, ExitStatus, Stdio};

use super::Config;

pub struct Clipper {
    cfg: Config,
}

impl<'a> Clipper {
    pub fn new(cfg: Config) -> Clipper {
        Clipper {
            cfg: cfg,
        }
    }

    pub fn run(cfg: Config) -> impl Future<Item = (), Error = ()> {
        listen(cfg)
    }

    pub fn from_args(matches: &'a clap::ArgMatches<'a>) -> Clipper {
        Clipper::new(Config::from_args(matches))
    }
}

pub fn listen(cfg: Config) -> impl Future<Item = (), Error = ()> + 'static {
    let tcp = TcpListener::bind(&cfg.addr()).unwrap();

    tcp.incoming().for_each(move |conn| {
        let (reader, _) = conn.split();
        let clipboard = clipboard_process(&cfg).spawn_async().unwrap();

        let future = write_to_clipboard(clipboard, reader)
            .map(|_| ())
            .map_err(|_| ());

        tokio::spawn(future);

        Ok(())
    }).map_err(|_| ())
}

fn clipboard_process(cfg: &Config) -> Command {
    let mut cmd = Command::new(
        cfg.clone().executable.expect("clipboard executable is not defined")
    );
    cmd.stdin(Stdio::piped());
    cmd
}

fn clipboard_process_exactname(name: &str) -> Command {
    let mut cmd = Command::new(name);
    cmd.stdin(Stdio::piped());
    cmd
}

fn write_to_clipboard<R>(mut cat: Child, reader: R) -> impl Future<Item = ExitStatus, Error = io::Error> + Send
where
    R: AsyncRead + Send
{
    let stdin = cat.stdin().take().unwrap();
    let write = tokio::io::copy(reader, stdin);

    write.and_then(|_| cat)
}
