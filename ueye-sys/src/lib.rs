#![allow(non_snake_case)]

pub mod aoi;
pub mod auto_parameter;
pub mod black_level;
pub mod boot_boost;
pub mod capture_configuration;
pub mod capture_status;
#[cfg(target_os = "windows")]
pub mod com_port;
pub mod configuration;
pub mod constants;
pub mod convert;
pub mod device_feature;
pub mod device_info;
pub mod edge_enhancement;
pub mod eth;
pub mod event;
pub mod exposure;
pub mod gamma;
pub mod image_buffer;
pub mod image_file;
pub mod io;
pub mod lut;
pub mod measure;
pub mod memory;
pub mod multicast;
pub mod optimal_camera_timing;
pub mod parameter_set;
pub mod persistent_memory;
pub mod pixel_clock;
pub mod power_delivery;
pub mod sequencer;
pub mod trigger;
pub mod types;
pub mod focus;
pub mod image_stabilization;
pub mod scene_preset;
pub mod zoom;
pub mod sharpness;
pub mod saturation;
pub mod trigger_debounce;
pub mod color_temperature;
pub mod direct_renderer;
pub mod hot_pixel;
pub mod transfer;
pub mod image_mem;
pub mod error;
pub mod color;
pub mod display;
pub mod video;
pub mod eeprom;
pub mod meta;

use constants::*;
use types::*;

//
//
// #[link(name = "ueye_api", kind = "dylib")]
// unsafe extern "C" {
