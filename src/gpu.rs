use glib::GString;
use gudev::{ClientExt, DeviceExt};
use std::fmt::Debug;


#[derive(Debug, Clone)]
pub struct GPU {
    pub name: String,
    pub device: String,
}

impl GPU {

    pub fn get_gpus() -> Vec<GPU> 
    {
        let mut gpus: Vec<GPU> = Vec::new();
        let client: gudev::Client = gudev::Client::new(&["drm"]);
        let devices: Vec<gudev::Device> = client.query_by_subsystem(Some("drm"));

        for device in devices {

            let device_file: Option<GString> = device.get_device_file();

            if device_file.is_some() {

                let device_file_uw: GString = device_file.unwrap();

                if device_file_uw.starts_with("/dev/dri/card") {

                    let parent: Option<gudev::Device> = device.get_parent();

                    if parent.is_some() {

                        let parent_uw: gudev::Device = parent.unwrap();

                        let card_name: GString = parent_uw
                            .get_property("ID_MODEL_FROM_DATABASE")
                            .unwrap_or(GString::from("<unknown>"));

                        gpus.push(GPU {
                            name: card_name.to_owned(),
                            device: device_file_uw.to_owned(),
                        })

                    }

                }

            }

        }
        return gpus;
    }

    pub fn pup_listen()
    {
        /* TODO */
    }

}
