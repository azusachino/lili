use lili::{
    executor::{ExecOptions, Executor, WindowsExecutor},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let options = ExecOptions::new("syncthing");
    let exec = WindowsExecutor::new(options);
    exec.exec()?;
    Ok(())
}
