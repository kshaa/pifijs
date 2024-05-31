use std::{env, path::PathBuf};
use pifijs_plotter_lib;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let linestrips_serialized = args.get(1).map(|s| { s.clone() });
    let image_path = args.get(2).map(|p| { PathBuf::from(p) });
    pifijs_plotter_lib::plotter::render(linestrips_serialized, image_path).await;
}
