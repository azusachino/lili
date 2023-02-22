use lili::Result;
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<()> {
    
    let output = Command::new("cmd")
        .args(&["/C", "dir"])
        .output()
        .await
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
