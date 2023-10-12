#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Browser {
    pub name: String,
    command: String,
}

impl Browser {
    pub fn open(self, args: Vec<String>) {
        std::process::Command::new(self.command)
            .args(args)
            .spawn()
            .expect("Failed to open browser.");
    }
}
