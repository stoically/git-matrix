use std::io;

use git_matrix::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <remote-name> <url>", args[0]);
        return Ok(());
    }
    eprintln!("args: {:?}", args);

    let _remote = args[1].clone();
    let url = args[2].clone();

    let git_matrix = GitMatrixBuilder::new(url).build().await.unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        eprintln!("input: {}", input);

        if input == "capabilities" {
            println!("push");
            println!("fetch");
            println!("");
        } else if input.starts_with("list") {
            let refs = git_matrix.refs().await.unwrap();
            if refs.len() > 0 {
                for (ref_name, ref_sha) in refs {
                    let remote_ref = format!("{} {}", ref_sha, ref_name);
                    println!("{}", remote_ref);
                }
                println!("@refs/heads/master HEAD");
            }
            println!("");
        } else if input.starts_with("push") {
            let push_args: Vec<&str> = input.split(" ").collect();
            let refs: Vec<&str> = push_args[1].split(":").collect();
            let src = refs[0];
            let dst = refs[1];
            git_matrix.push(src, dst).await.unwrap();
            println!("ok {}", dst);

            println!("");
        } else if input.starts_with("fetch") {
            git_matrix.fetch().await.unwrap();
            println!("");
        } else if input == "" {
            break;
        }
    }

    Ok(())
}
