use clap::Parser;
use command_def::types::cmd_types::{GenPassOpts, Opts, SubCommand};
use gene_pwd::process::pwd_process;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::GenPass(opts) => {}
    }
    let pwd = pwd_process::generate_password(16, true, true, true);
    println!("pwd = {}", pwd);
    Ok(())
}
