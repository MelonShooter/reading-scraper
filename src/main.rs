pub mod text_fetchers;
pub(crate) mod util;

fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()
        .unwrap()
        .block_on(async {
            println!("Hello world");
        });
}
