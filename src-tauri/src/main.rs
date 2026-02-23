// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod logic;
mod tools;

fn main() {
    holairs_rockbox_manager_lib::run()
}
