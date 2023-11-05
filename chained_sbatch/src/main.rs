use std::fmt::{Display, Formatter};

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Dependency {
    AfterOK,
    AfterNotOK,
    AfterAll,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Dependency::AfterOK => write!(f, "afterok"),
            Dependency::AfterNotOK => write!(f, "afternotok"),
            Dependency::AfterAll => write!(f, "afterany"),
        }
    }
}

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file: String,
    #[clap(short, long)]
    iters: usize,
    // options are afterok, afternotok, afterany
    #[clap(value_enum, default_value_t=Dependency::AfterOK, long)]
    dep: Dependency,
}
fn main() {
    let args = Args::parse();
    // sbatch the first script

    let result = std::process::Command::new("sbatch")
        .arg(&args.file)
        .output();

    let mut job_id = match result {
        Ok(_) => String::from_utf8(result.unwrap().stdout)
            .unwrap()
            .split_whitespace()
            .nth(3)
            .unwrap()
            .to_string(),
        Err(e) => {
            println!("Error: {}", e);
            panic!("Error: {}", e);
        }
    };
    println!("job_id: {}", job_id);

    // This is the jobid of the first job

    for idx in 1..args.iters {
        println!("Submitting iter {}", idx);

        let result = std::process::Command::new("sbatch")
            .arg(format!("--dependency={}:{}", args.dep, job_id.trim_start()))
            .arg(&args.file)
            .output();

        job_id = match result {
            Ok(_) => String::from_utf8(result.unwrap().stdout)
                .unwrap()
                .split_whitespace()
                .nth(3)
                .unwrap()
                .to_string(),
            Err(e) => panic!("Error: {}", e),
        };
        println!("job_id: {}", job_id);
    }
}
