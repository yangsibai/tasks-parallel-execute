extern crate crossbeam;
extern crate num_cpus;

use crossbeam::thread;
use std::io::{self, BufRead};
use std::process::Command;
use std::time::Duration;

fn main() {
    let cpu_num = num_cpus::get();

    let mut tasks = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        tasks.push(line.unwrap());
    }

    thread::scope(|scope| {
        for i in 0..cpu_num {
            let my_tasks = tasks
                .iter()
                .skip(i)
                .step_by(cpu_num)
                .collect::<Vec<&String>>();
            scope.spawn(move |_| {
                for task in my_tasks {
                    println!("{}", task);
                    let cmds = parse_cmds(task);
                    execute_commands(&cmds);
                }
            });
        }
    })
    .unwrap();
}

#[derive(Debug)]
struct Cmd(String, Vec<String>);

fn execute_commands(cmds: &Vec<Cmd>) {
    for cmd in cmds {
        let output = Command::new(&cmd.0)
            .args(&cmd.1)
            .output()
            .expect("Failed to execute process.");
        if !output.status.success() {
            println!("{:?}", output);
        }
    }
}

fn parse_cmds(cmd: &str) -> Vec<Cmd> {
    let v: Vec<&str> = cmd.trim().split("&&").collect();
    let mut cmds: Vec<Cmd> = vec![];
    for c in v {
        let vec: Vec<&str> = c.trim().split(" ").collect();
        let cmdName = vec.first().unwrap().to_string();
        let args: Vec<String> = vec.iter().skip(1).map(|e| e.to_string()).collect();
        cmds.push(Cmd(cmdName, args));
    }
    return cmds;
}
