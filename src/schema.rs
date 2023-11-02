use clap::Parser;
use serde::{Deserialize, Serialize};

/// cli struct
#[derive(Parser, Debug)]
#[command(name = "rust-tls-microservice")]
#[command(author = "Luigi Mario Zuccarelli <luigizuccarelli@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "A simple microservice to be used as a basis for the UPaaS (Unikernel based Platform As A Service)", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// use tls/https default is false
    #[arg(short, long, value_name = "tls", default_value = "false")]
    pub tls: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VSBase {
    pub name: String,

    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "patientId")]
    pub patient_id: String,

    pub data: Vec<VitalSigns>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VitalSigns {
    pub hr: i64,
    pub bps: i64,
    pub bpd: i64,
    pub spo2: i64,
    pub custom: Custom,
    pub date: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub tp: f64,
    pub rr: i64,
    pub etc: String,
}

#[derive(Serialize)]
pub struct VSResponse {
    pub device_id: String,
    pub patient_id: String,
    pub status: bool,
}

impl VSResponse {
    pub fn res(data: VSBase) -> VSResponse {
        VSResponse {
            device_id: data.device_id,
            patient_id: data.patient_id,
            status: true,
        }
    }
}

#[derive(Serialize)]
pub struct IsAlive {
    pub name: String,
    pub version: String,
    pub status: String,
}
