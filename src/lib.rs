#![feature(proc_macro_hygiene)]
#![feature(with_options)]
#![feature(const_mut_refs)]
#![feature(exclusive_range_pattern)]
#![allow(clippy::borrow_interior_mutable_const, clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc, clippy::wrong_self_convention)]

pub mod common;
mod hazard_manager;
mod hitbox_visualizer;
mod training;

#[cfg(test)]
mod test;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate num_derive;

use crate::common::*;
use crate::menu::set_menu_from_url;

use skyline::libc::mkdir;
use std::fs;
use skyline::nro::{self, NroInfo};

use owo_colors::OwoColorize;

fn nro_main(nro: &NroInfo<'_>) {
    if nro.module.isLoaded {
        return;
    }

    if nro.name == "common" {
        skyline::install_hooks!(
            training::shield::handle_sub_guard_cont,
            training::directional_influence::handle_correct_damage_vector_common,
            training::sdi::process_hit_stop_delay,
            training::tech::handle_change_status,
        );
    }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

#[skyline::main(name = "training_modpack")]
pub fn main() {
    macro_rules! log {
        ($($arg:tt)*) => {
            print!("{}", "[Training Modpack] ".green());
            println!($($arg)*);
        };
    }

    log!("Initialized.");

    // HTTP endpoint
    let host = "https://my-project-1511972643240-default-rtdb.firebaseio.com";
    let path = "/users/jack/name.json";
    
    let url = format!("{}{}", host, path);
    let response: minreq::Response = minreq::get(url)
        .send()
        .ok()
        .unwrap()
        // .json()
        // .unwrap()
        ;

    println!("response: {:?}", response);

    hitbox_visualizer::hitbox_visualization();
    hazard_manager::hazard_manager();
    training::training_mods();
    nro::add_hook(nro_main).unwrap()
     
    unsafe {
        mkdir(c_str!("sd:/TrainingModpack/"), 777);
    }

    let ovl_path = "sd:/switch/.overlays/ovlTrainingModpack.ovl";
    if fs::metadata(ovl_path).is_ok() {
        log!("Removing ovlTrainingModpack.ovl...");
        fs::remove_file(ovl_path).unwrap();
    }

    log!("Performing version check...");
    release::version_check();

    let menu_conf_path = "sd:/TrainingModpack/training_modpack_menu.conf";
    if fs::metadata(menu_conf_path).is_ok() {
        log!("Loading previous menu from training_modpack_menu.conf...");
        let menu_conf = fs::read(menu_conf_path).unwrap();
        if menu_conf.starts_with(b"http://localhost") {
           set_menu_from_url(std::str::from_utf8(&menu_conf).unwrap());
        }
    }
}
