use std::env;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn render_plot(image_path: &PathBuf, serialized_plot: &String) {
    let renderer = env::var("PIFIJS_RENDERER").expect("Expected PIFIJS_RENDERER in the environment");
    let result = Command::new(renderer)
        .arg(&serialized_plot)
        .arg(&image_path)
        .output()
        .await;
    let output = result.expect("Renderer seems to have failed");
    let stdout = std::str::from_utf8(output.stdout.as_ref()).unwrap();
    let stderr = std::str::from_utf8(output.stderr.as_ref()).unwrap();
    println!("{}", stdout);
    println!("{}", stderr);
}
