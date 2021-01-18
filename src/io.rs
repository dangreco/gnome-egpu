use crate::gpu::GPU;
use std::fs::File;
use std::io::prelude::*;
use std::process;

pub struct IO {}

impl IO {
    pub fn set_udev_rules(gpu: &GPU) -> std::io::Result<()> 
    {
        let mut mutter_rule_file =
            File::create("/usr/lib/udev/rules.d/61-mutter-primary-gpu.rules")?;

        let mutter_rule_contents: String = format!(
            "ENV{{DEVNAME}}==\"{}\", TAG+=\"mutter-device-preferred-primary\"",
            gpu.device
        );

        mutter_rule_file.write_all(mutter_rule_contents.as_bytes())?;
        mutter_rule_file.sync_all()?;
        /*

        TODO: Write hotplugging rule to auto-restart GDM when GPU is plugged in.

        let mut udev_rule_file = File::create("/etc/udev/rules.d/99-egpu-hotplug.rules")?;
        let udev_rule_contents: String = format!();

        udev_rule_file.write_all(udev_rule_contents.as_bytes())?;
        udev_rule_file.sync_all()?;
        */

        process::Command::new("udevadm")
            .args(&["control", "--reload-rules"])
            .spawn();

        process::Command::new("udevadm").args(&["trigger"]).spawn();

        Ok(())
    }

    pub fn cleanup() -> std::io::Result<()> {
        std::fs::remove_file("/usr/lib/udev/rules.d/61-mutter-primary-gpu.rules")?;
        Ok(())
    }

}
