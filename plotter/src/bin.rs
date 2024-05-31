use std::{env, path::PathBuf};
use pifijs_plotter_lib;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = args.get(1).map(|p| { PathBuf::from(p) });
    pifijs_plotter_lib::plotter::render(image_path).await;
}
