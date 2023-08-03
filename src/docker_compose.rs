use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

pub struct DockerCompose {
    file: String,
    logs_dir: String,
}

impl DockerCompose {
    pub fn new(file: &str, logs_dir: &str) -> DockerCompose {
        DockerCompose {
            file: file.to_string(),
            logs_dir: logs_dir.to_string(), 
        }
    }

    pub fn up(&self) {
        let output = Command::new("docker-compose")
            .arg("-f")
            .arg(self.file.clone())
            .arg("up")
            .arg("-d")
            .output()
            .expect("Failed to execute command");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Errors: {}", String::from_utf8_lossy(&output.stderr));
        println!("Starting Test");

        let dir = &self.logs_dir;
        if Path::new(dir).exists() {
            std::fs::remove_dir_all(dir).unwrap();
        }
        std::fs::create_dir_all(dir).unwrap();

        let output = Command::new("docker")
            .arg("ps")
            .arg("--format")
            .arg("{{.Names}}")
            .output()
            .unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        let containers: Vec<String> = stdout.lines().map(String::from).collect();

        let _handles: Vec<_> = containers
            .into_iter()
            .map(|container| {
                let file_name = format!("{}/{}.log", dir, container);
                let file_path = std::path::PathBuf::from(file_name);
                let docker_compose_file = self.file.clone();
                thread::spawn(move || DockerCompose::follow_container_log(docker_compose_file, container, file_path))
            })
            .collect();
    }

    pub fn down(&self) {
        let _output = Command::new("docker-compose")
            .arg("-f")
            .arg(self.file.clone())
            .arg("down")
            .output()
            .expect("Failed to execute command");

        println!("Gracefully shutting down...");
    }

    fn follow_container_log(docker_compose_file: String, container: String, file_path: std::path::PathBuf) {
        let file = File::create(file_path).unwrap();
        let _ = Command::new("docker-compose")
            .arg("-f")
            .arg(docker_compose_file)
            .arg("logs")
            .arg("--follow")
            .arg("--no-log-prefix")
            .arg(&container)
            .stdout(Stdio::from(file))
            .spawn()
            .unwrap();
    }
}
