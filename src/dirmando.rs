use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use std::io::{stdin,stdout,Write};
use std::process::{Command, Stdio};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dirmando {
    pub database: HashMap<String, Vec<String>>
}

pub fn fromFile(filePath: &str) -> Dirmando {
    let mut f = File::open(filePath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    serde_json::from_str(&contents).unwrap()
}

impl Dirmando {

    pub fn save(&self, currentDirectory: &String, command: &String) {

    }

    pub fn load(&self, currentDirectory: &String) -> Option<&Vec<String>> {
        self.database.get(currentDirectory)
    }

    pub fn choose(&self, commandos: &Vec<String>) {
        for (index, command) in commandos.iter().enumerate() {
            println!("{} - {}", index, command);
        }
        let mut s= String::new();
        let _= stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }
        
        let c = &commandos[s.parse::<usize>().unwrap()];

        let mut cmd = Command::new(c)
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .spawn()
                        .unwrap();

        let status = cmd.wait();
        println!("Exited with status {:?}", status);
    }

    pub fn import(&self) {

    }

    pub fn export(&self) {

    }
}
