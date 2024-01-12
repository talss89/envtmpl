use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use glob::glob;

mod template;

/// Compile Go-style templates, exposing environment variables à la dockerize.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input path / file to parse template(s) from, and output path / file
    #[arg(short, long, required=true, num_args=1.., value_name = "INPUT:OUTPUT")]
    target: Vec<String>,

    /// Overwrite existing output files
    #[arg(short, long)]
    overwrite: bool,
}

fn process(path: &PathBuf, outpath: &PathBuf) -> anyhow::Result<()> {
    println!("{} -> {}", &path.to_str().unwrap(), &outpath.to_str().unwrap());

    let output = crate::template::render(fs::read_to_string(&path)?)?;

    fs::write(&outpath, &output)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for target in args.target {
        let parts: Vec<&str> = target.split(":").collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("--target must be in the format [INPUT]:[OUTPUT]"));
        }

        let input = PathBuf::from_str(parts[0])?;
        let output = PathBuf::from_str(parts[1])?;

        if !fs::metadata(&input).is_ok() {
            return Err(anyhow::anyhow!("[INPUT] does not exist"));
        }

        if fs::metadata(&input)?.is_dir() && fs::metadata(&output).is_ok() && fs::metadata(&output).unwrap().is_file() {
            return Err(anyhow::anyhow!("If [INPUT] is a directory, [OUTPUT] must also be a directory"));
        }
        
        if fs::metadata(&output).is_ok() && !args.overwrite {
            return Err(anyhow::anyhow!("[OUTPUT] already exists. Pass --overwrite to overwrite."));
        }

        if fs::metadata(&input)?.is_file() {
            process(&input, &output)?;
        } else {

            if !fs::metadata(&output).is_ok() {
                fs::create_dir_all(&output)?;
            }

            let input_pattern = &input.join("**/*").to_owned();

            for entry in glob(input_pattern.to_str().unwrap()).expect("Failed to read glob pattern") {
                
                match entry {
                    Ok(path) => {
                        if fs::metadata(&path)?.is_file() {
                            let output = &output.join(&path);
                            process(&path, &output)?;
                        }
                    }

                    Err(e) => {
                        return Err(anyhow::anyhow!(e));
                    }
                }
                
                
            }
        }

        

    }

    Ok(())
    
}