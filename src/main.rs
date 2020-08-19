use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();
    let _token = dotenv::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN is not set");
    // let tmpdir = tempdir().expect("Failed to create temp dir");
    // log::info!("Temp dir: {}", tmpdir.path().display());
    // tokio::run(handle_updates(UpdateMethod::poll(api.clone()), Handler { api, tmpdir }));
}
