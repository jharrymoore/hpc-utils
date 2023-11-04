use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file: String,
    #[clap(short, long)]
    iters: usize,
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
            .arg(format!("--dependency=afterok:{}", job_id.trim_start()))
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
