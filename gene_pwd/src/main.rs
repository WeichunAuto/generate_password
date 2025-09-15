use clap::Parser;
use command_def::types::cmd_types::{GenPassOpts, Opts, SubCommand};
use gene_pwd::{process::pwd_process, score::mark_score::evoluate_score};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::GenPass(opts) => {}
    }
    let pwd = pwd_process::generate_password(12, true, true, true);
    eprintln!("pwd = {}", pwd);
    let strenth = evoluate_score(pwd.as_str());
    println!("strength = {}", strenth);
    Ok(())
}
