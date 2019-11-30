use dotenv::dotenv;
use log;
use std::{env, path::Path};
use tgbot::{
    handle_updates,
    methods::GetFile,
    types::{Document, MessageData, Update, UpdateKind},
    Api, Config, UpdateHandler, UpdateMethod,
};
use tempfile::{tempdir, TempDir};
use tokio::{fs::File, io::AsyncWrite};
use futures::Future;
use failure::Error;

struct Handler {
    api: Api,
    tmpdir: TempDir,
}

fn handle_document(api: &Api, tmpdir: &Path, document: Document) -> Box<dyn Future<Item = (), Error = Error> + Send> {
    let api = api.clone();
    let path = tmpdir.join(document.file_name.clone().unwrap_or_else(|| String::from("unknown")));
    Box::new(
        api.execute(GetFile::new(document.file_id.as_str()))
            .and_then(move |file| {
                let file_path = file.file_path.unwrap();
                api.download_file(file_path)
            })
            .and_then(move |data| {
                println!("Name: {:?}", document.file_name);
                println!("Mime-Type: {:?}", document.mime_type);
                println!("Document size: {:?}", document.file_size);
                println!("Downloaded size: {:?}", data.len());
                File::create(path)
                    .and_then(move |mut file| file.poll_write(&data))
                    .map(|_| ())
                    .from_err()
            }),
    )
}

impl UpdateHandler for Handler {
    fn handle(&mut self, update: Update) {
        log::info!("got an update: {:?}\n", update);
        if let UpdateKind::Message(message) = update.kind {
            if let MessageData::Document { data, .. } = message.data {
                self.api.spawn(handle_document(&self.api, self.tmpdir.path(), data));
            }
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();
    let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    let proxy = env::var("SOCKS_PROXY").expect("SOCKS_PROXY is not set");
    let mut config = Config::new(token);
    config = config.proxy(proxy);
    let api = Api::new(config).expect("Failed to create API");
    let tmpdir = tempdir().expect("Failed to create temp dir");
    log::info!("Temp dir: {}", tmpdir.path().display());
    tokio::run(handle_updates(UpdateMethod::poll(api.clone()), Handler { api, tmpdir }));
}
