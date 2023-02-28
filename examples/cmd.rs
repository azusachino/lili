use std::fs::File;
use std::process::{Command, Stdio};

fn main() {
    // Define a list of applications to run
    let apps = vec![
        ("app1", "/path/to/app1", "/path/to/app1.log"),
        ("app2", "/path/to/app2", "/path/to/app2.log"),
        ("app3", "/path/to/app3", "/path/to/app3.log"),
        // add more apps here
    ];

    // Create a vector to hold the child processes
    let mut children = vec![];

    // Iterate over the list of applications and run them
    for (name, path, log_path) in apps {
        // Create a new file to write the log output to
        let log_file = File::create(log_path).expect("Failed to create log file");

        // Spawn the child process and redirect its stdout to the log file
        let child = Command::new(path)
            .stdout(Stdio::from(log_file))
            .spawn()
            .expect(&format!("Failed to start {}", name));

        // Add the child process to the vector of children
        children.push(child);
    }

    // Wait for all the child processes to complete
    for child in children.iter_mut() {
        child.wait().expect("Failed to wait for child process");
    }
}
