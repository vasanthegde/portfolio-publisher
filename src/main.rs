use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::Command;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let args: &[String] = &args[1..];
    match args.first() {
        Some(post) => {
    
            let file = File::open("post.html")?;
            let mut reader = BufReader::new(file);
        
            let mut html = String::new();
            reader.read_to_string(&mut html)?;
        
            let post = format!("<p>{}</p>", post);
            let updated_html = html.replace(
                "<div class=\"posts\">",
                &format!("<div class=\"posts\">\n{}\n", post),
            );
        
            //  update the file with post
            let mut output_file = File::create("post.html")?;
            write!(output_file, "{}", updated_html)?;
            println!("Successfully updated the post !!");
            git();
            deploy();
        }
        None => {
            deploy();
        }
    }
   


    Ok(())
}

fn deploy() {
    //  wrangler pages publish . --project-name space-turtle
    let output = Command::new("wrangler")
        .arg("pages")
        .arg("publish")
        .arg(".")
        .arg("--project-name")
        .arg("space-turtle")
        .output()
        .expect("Failed to run wrangler publish command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn git() {
    // git add
    Command::new("git")
        .arg("add")
        .arg("post.html")
        .output()
        .expect("Failed to run git add command");
    // git commit
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Adding post by CLI")
        .output()
        .expect("Failed to run git commit command");
    //  git push
    Command::new("git")
        .arg("push")
        .output()
        .expect("Failed to run git push command");
}
