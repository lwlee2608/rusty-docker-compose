use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

#[derive(Clone)]
pub struct DockerComposeCmd {
    file: String,
    logs_dir: String,
}

impl DockerComposeCmd {
    pub fn new(file: &str, logs_dir: &str) -> DockerComposeCmd {
        DockerComposeCmd {
            file: file.to_string(),
            logs_dir: logs_dir.to_string(),
        }
    }

    fn get_docker_compose_command() -> &'static str {
        if Command::new("docker-compose").output().is_ok() {
            return "docker-compose";
        } else {
            return "docker compose";
        }
    }

    pub fn up(&self) {
        let docker_compose_command = Self::get_docker_compose_command();

        let output = Command::new(docker_compose_command)
            .arg("-f")
            .arg(self.file.clone())
            .arg("up")
            .arg("-d")
            .output()
            .expect("Failed to execute command");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        println!("Errors: {}", String::from_utf8_lossy(&output.stderr));
        println!("Docker Compose started");

        let dir = &self.logs_dir;
        if Path::new(dir).exists() {
            std::fs::remove_dir_all(dir).unwrap();
        }
        std::fs::create_dir_all(dir).unwrap();

        let output = Command::new(docker_compose_command)
            .arg("-f")
            .arg(self.file.clone())
            .arg("ps")
            .arg("--services")
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
                thread::spawn(move || {
                    let follow_container_log =
                        |container: String, file_path: std::path::PathBuf| {
                            let file = File::create(file_path).unwrap();
                            let _ = Command::new(docker_compose_command)
                                .arg("-f")
                                .arg(docker_compose_file)
                                .arg("logs")
                                .arg("--follow")
                                .arg("--no-log-prefix")
                                .arg(&container)
                                .stdout(Stdio::from(file))
                                .spawn()
                                .unwrap();
                        };

                    follow_container_log(container, file_path);
                });
            })
            .collect();
    }

    pub fn down(&self) {
        println!("Gracefully shutting down...");

        let docker_compose_command = Self::get_docker_compose_command();

        let _output = Command::new(docker_compose_command)
            .arg("-f")
            .arg(self.file.clone())
            .arg("down")
            .output()
            .expect("Failed to execute command");
    }
}

pub struct DockerCompose {
    cmd: DockerComposeCmd,
}

impl DockerCompose {
    pub fn new(file: &str, logs_dir: &str) -> DockerCompose {
        let cmd = DockerComposeCmd::new(file, logs_dir);
        cmd.up();
        DockerCompose { cmd }
    }
}

impl Drop for DockerCompose {
    fn drop(&mut self) {
        self.cmd.down();
    }
}
