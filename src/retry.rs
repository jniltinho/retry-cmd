use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use colored::Colorize;

#[derive(Debug)]
pub struct RetryConfig<'a> {
    pub max: u32,
    pub interval: Duration,
    pub expected_exitcode: i32,
    pub quiet: bool,
    pub cmd: Vec<&'a str>,
}

pub fn retry(config: RetryConfig) -> i32 {
    let mut exit_code = 0;
    let mut i = 1;
    while i <= config.max || config.max == 0 {
        let status = match Command::new(&config.cmd[0])
            .args(&config.cmd[1..config.cmd.len()])
            .stdin(if config.quiet {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stdout(if config.quiet {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .stderr(if config.quiet {
                Stdio::null()
            } else {
                Stdio::inherit()
            })
            .status()
        {
            Ok(s) => s,
            Err(err) => panic!("{} {}", "Failed to execute command:".red(), err),
        };
        exit_code = status.code().unwrap();
        match status.code() {
            Some(code) if code == config.expected_exitcode => {
                println!("{}", "Successfully ran command. Abort retry.".green());
                break;
            }
            Some(code) => println!("{}", format!("[Retry {i}] Command failed with exit code {code}").yellow()),
            None => println!("{}",
                format!("[Retry {i}] Command failed because it was termianted by a signal").red(),
            ),
        }

        if i != config.max {
            let f = format!("{} {:?} {}...","Waiting", config.interval, "seconds").purple();
            println!("{}",f);
            thread::sleep(config.interval);
        }

        i += 1;
    }
  exit_code
}
