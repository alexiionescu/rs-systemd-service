mod utils;
use clap::Parser as _;
use std::{borrow::Cow, path::PathBuf};

use crate::utils::MaybeReplaceExt as _;

fn main() {
    let cli = Cli::parse();
    let workingdir = cli
        .workdir
        .canonicalize()
        .unwrap_or_else(|_| {
            panic!(
                "Failed to canonicalize working directory: {}",
                cli.workdir.display()
            );
        })
        .to_str()
        .unwrap_or_else(|| {
            panic!(
                "Working directory path is not valid UTF-8: {}",
                cli.workdir.display()
            );
        })
        .to_owned();
    let target_path = cli.target.canonicalize().unwrap_or_else(|_| {
        panic!(
            "Failed to canonicalize target executable: {}",
            cli.target.display()
        );
    });
    let target_basename = target_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_else(|| {
            panic!(
                "Target executable path does not have a valid file name: {}",
                cli.target.display()
            );
        });
    let target = target_path
        .to_str()
        .unwrap_or_else(|| {
            panic!(
                "Target executable path is not valid UTF-8: {}",
                cli.target.display()
            );
        })
        .to_owned();
    let template = std::fs::read_to_string(&cli.template_file).unwrap_or_else(|_| {
        panic!(
            "Failed to read template file: {}",
            cli.template_file.display()
        );
    });
    let userline = cli
        .username
        .as_deref()
        .map_or("".to_owned(), |s| format!("User={s}"));
    let group_line = cli
        .usergroup
        .as_deref()
        .map_or("".to_owned(), |s| format!("Group={s}"));
    let args_escaped = cli
        .args
        .iter()
        .map(|arg| {
            if arg.contains(' ') {
                format!("\"{arg}\"")
            } else {
                arg.to_string()
            }
        })
        .collect::<Vec<_>>();
    let default_description = format!("{} {} {}", cli.name, target_basename, cli.args.join(" "));
    let description = cli.description.as_deref().unwrap_or(&default_description);
    let syslog_id = cli.syslog_id.as_deref().unwrap_or(&cli.name);
    let output = Cow::from(&template)
        .maybe_replace("<%name%>", &cli.name)
        .maybe_replace("<%workdir%>", &workingdir)
        .maybe_replace("<%target%>", &target)
        .maybe_replace("<%user-line%>", &userline)
        .maybe_replace("<%usergr-line%>", &group_line)
        .maybe_replace("<%description%>", description)
        .maybe_replace("<%syslogid%>", syslog_id)
        .maybe_replace("<%restart%>", &cli.restart)
        .maybe_replace("<%restart_after%>", &cli.restart_after.to_string())
        .maybe_replace("<%args%>", &args_escaped.join(" "));

    println!("\n------\n{}\n---------\n", output);

    eprintln!("\nWriting service file to: {}.service", cli.name);
    let output_path = cli.name.clone() + ".service";
    std::fs::write(&output_path, output.as_bytes()).unwrap_or_else(|_| {
        panic!("Failed to write service file: {}", output_path);
    });

    eprintln!("You can now copy it to /etc/systemd/system/ and run `systemctl daemon-reload`.\n");
    eprintln!(
        "sudo mv {} /etc/systemd/system/{}.service",
        output_path, cli.name
    );
    eprintln!("sudo systemctl daemon-reload");
    eprintln!("sudo systemctl enable --now {}", cli.name);
    eprintln!("sudo systemctl status {}", cli.name);
    eprintln!("\njournalctl -xeu {}", cli.name);
}

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(
        short,
        long,
        help = "service name. this will be used for output file name"
    )]
    name: String,

    #[clap(long, short, help = "executable path")]
    target: PathBuf,

    #[clap(
        long,
        help = "template file for the service",
        default_value = "rust.service.templ"
    )]
    template_file: PathBuf,

    #[clap(
        long,
        help = "user name, if any, User=<%username%> line will replace <%user-line%>. If not specified, the service will run as root."
    )]
    username: Option<String>,

    #[clap(
        long,
        help = "user group, if any, Group=<%usergroup%> line will replace <%usergr-line%>. If not specified, the service will run as root."
    )]
    usergroup: Option<String>,

    #[clap(
        long,
        help = "service description Description=<%description%> (Default: service name + target basename + arguments)"
    )]
    description: Option<String>,

    #[clap(
        long,
        help = "syslog identifier SyslogIdentifier=<%syslogid%> (Default: service name)"
    )]
    syslog_id: Option<String>,

    #[clap(
        long,
        help = "working directory for the service. WorkingDirectory=<%workdir%>",
        default_value = "."
    )]
    workdir: PathBuf,

    #[clap(
        long,
        help = "restart option Restart=<%restart%>",
        default_value = "always"
    )]
    restart: String,

    #[clap(
        long,
        help = "restart after failure time in seconds. RestartSec=<%restart_after%>",
        default_value_t = 30
    )]
    restart_after: u64,

    #[clap(last = true, help = "command line arguments for the target executable")]
    args: Vec<String>,
}
