#[derive(Debug)]
pub struct MobileData {
    pub url: String,
    pub title: String,
    pub os: String,
    pub size: String,
    pub cpu: String,
    pub gpu: String,
    pub has_fingerprint: bool,
    pub has_nfc: bool,
    pub has_dual_sim: bool,
    pub back_camera: Vec<String>,
    pub front_camera: String,
    pub card_slot: String,
    pub usb: String,
    pub battery: String,
    pub storage: Vec<String>,
    pub chipset: String,
    pub price: u32,
    pub quantity: u8,
}

pub fn get_gsm_attributes() -> [String;15] {
    return [
        "modelname", 
        "os", 
        "displaysize", 
        "cpu", 
        "gpu", 
        "sensors",
        "nfc",
        "sim",
        "cam1modules", // back
        "cam2modules", // front
        "memoryslot", 
        "usb",
        "batdescription1",
        "internalmemory",
        "chipset"
    ].map(
        |x| -> String { String::from(x)}
    )
}

impl MobileData {

    pub fn define_value(&mut self, key: &String, value: &String) {
        
        match key.as_str() {
            // "url" => self.url = value.to_string(),
            "modelname" => self.title = value.to_string(),
            "os" => self.os = value.to_string(),
            "displaysize" => self.size = value.to_string(),
            "cpu" => self.cpu = value.to_string(),
            "gpu" => self.gpu = value.to_string(),
            "sensors" => self.has_fingerprint = value.to_string().to_lowercase().contains("fingerprint"),
            "nfc" => {
                let v = value.to_string().to_lowercase();
                self.has_nfc = v.contains("nfc") || v.contains("yes")
            }
            "sim" => self.has_dual_sim = value.to_string().to_lowercase().contains("dual"),
            "cam1modules" => {
                self.back_camera = value.split(",").map(
                    |x| -> String { String::from(x)}
                ).collect();
            },
            "cam2modules" => self.front_camera = value.to_string(),
            "memoryslot" => self.card_slot = value.to_string(),
            "usb" => self.usb = value.to_string(),
            "batdescription1" => self.battery = value.to_string(),
            "internalmemory" => {
                self.storage=value.split(",").map(
                    |x| -> String { String::from(x)}
                ).collect();
            },
            "chipset" => self.chipset = value.to_string(),
            "price" => self.price = value.parse::<u32>().unwrap(),
            "quantity" => self.quantity = value.parse::<u8>().unwrap(),
            _ => {}
        }
    }
}
