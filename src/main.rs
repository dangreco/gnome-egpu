use std::env;
use std::process;
use std::io::{BufReader, BufRead};
use std::os::unix::process::ExitStatusExt;

use glib::GString;

use dialoguer::{
    Select,
    Confirm,
    theme::ColorfulTheme
};

mod gpu;
use gpu::GPU;

mod io;
use io::IO;

fn setup()
{
    if sudo::check() != sudo::RunningAs::Root {
        println!("Please run this script as root!");
        process::exit(2);
    }

    let gpus: Vec<GPU> = GPU::get_gpus();

    if gpus.len() < 2 {
        println!("You only have 1 GPU installed. Are you sure the other one(s) is/are plugged in?");
        process::exit(3);
    }

    let items: Vec<String> = gpus.clone().into_iter().map(|g| g.name).collect();

    let selection: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select your eGPU:")
        .items(&items)
        .interact().unwrap();

    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you wish to continue?")
        .wait_for_newline(true)
        .interact()
        .unwrap();

    if proceed {

        println!("Setting udev rules...");

        if IO::set_udev_rules(&gpus.get(selection).unwrap()).is_ok() {

            println!("Done!");
            println!("Run \"# gnome-egpu pup\" when plugging in/removing the eGPU.");

            let restart = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("You need to restart GDM to notice the changes right now, restart GDM?")
                .wait_for_newline(true)
                .interact()
                .unwrap();

            if restart {
                process::Command::new("systemctl")
                    .args(&[
                        "restart",
                        "gdm.service"
                    ])
                    .spawn();
            } else {
                println!("Ok!");
            }

        }

    }

}

fn pup()
{
    println!("Waiting for eGPU plug/unplug event...");

    GPU::pup_listen();
}

fn cleanup()
{
    if sudo::check() != sudo::RunningAs::Root {
        println!("Please run this script as root!");
        process::exit(2);
    }

    IO::cleanup();
}

fn list_blacklisted()
{
    let modprobe_result = process::Command::new("/usr/sbin/modprobe")
        .args(&["--showconfig"])
        .output();
    if modprobe_result.is_err() {
        println!("Warn: /usr/sbin/modprobe failed");
        println!("{}", modprobe_result.unwrap_err());
        return
    }
    let modprobe = modprobe_result.unwrap();
    if modprobe.status != std::process::ExitStatus::from_raw(0) {
        println!("Warn: /usr/sbin/modprobe failed");
        println!("{}", std::str::from_utf8(modprobe.stderr.as_slice()).unwrap());
        return
    }

    let reader = BufReader::new(modprobe.stdout.as_slice());
    for line in reader.lines() {
        let ln = line.unwrap();
        let lnr : &str = ln.as_ref();
        if lnr.starts_with("blacklist ") {
            let module = lnr.strip_prefix("blacklist ").unwrap();
            if module == "radeon" || module == "amdgpu" ||
                module.contains("nvidia") || module == "nouveau"
            {
                println!("Warn: GPU module `{}' is blacklisted", module)
            }
        }
    }
}

fn list()
{
    list_blacklisted();
    for gpu in GPU::get_gpus() {
        println!("name={}, device={}", gpu.name, gpu.device);
    }
}

fn help()
{
    println!(r#"Available commands:

  setup  -  Set up gnome-egpu.
  pup  -  "(P)lug/(U)n(P)lug": run before plugging in/unplugging eGPU.
  cleanup  -  Remove all files created by gnome-egpu.
  list - print available GPUs, warn about blacklisted modules
"#);
}

fn print_error()
{

    println!("Use gnome-egpu as follows:\n\n # gnome-egpu <command>\n\nUse 'help' as a command for more information.");
    process::exit(1);
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_error();
    }

    match args.get(1).unwrap().as_str() {

        "setup" => setup(),
        "pup" => pup(),
        "cleanup" => cleanup(),
        "list" => list(),

        _ => help()

    }

}
