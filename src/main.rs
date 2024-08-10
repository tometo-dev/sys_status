#[macro_use]
extern crate rocket;

use rocket::serde::json::serde_json;
use std::string::String;

#[get("/")]
fn index() -> String {
    serde_json::to_string(&get_system_status()).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

extern crate systemstat;

use std::collections::HashMap;

use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

fn get_system_status() -> HashMap<String, String> {
    let mut system_status = HashMap::new();
    let sys = System::new();

    match sys.memory() {
        Ok(mem) => {
            system_status.insert(
                String::from("memory"),
                format!(
                    "{} used / {} ({} bytes) total",
                    saturating_sub_bytes(mem.total, mem.free),
                    mem.total,
                    mem.total.as_u64()
                ),
            );
        }
        Err(x) => println!("\nMemory: error: {}", x),
    }

    match sys.uptime() {
        Ok(uptime) => {
            let seconds = uptime.as_secs() % 60;
            let minutes = (uptime.as_secs() / 60) % 60;
            let hours = (uptime.as_secs() / 60) / 60;
            system_status.insert(
                String::from("uptime"),
                format!("{}h:{}m:{}s", hours, minutes, seconds),
            );
        }
        Err(x) => println!("\nUptime: error: {}", x),
    }

    match sys.boot_time() {
        Ok(boot_time) => {
            system_status.insert(String::from("boot_time"), format!("{}", boot_time));
        }
        Err(x) => println!("\nBoot time: error: {}", x),
    }

    match sys.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu = cpu.done().unwrap();
            system_status.insert(
                String::from("cpu_load"),
                format!(
                    "{}% user, {}% nice, {}% system, {}% intr, {}% idle ",
                    cpu.user * 100.0,
                    cpu.nice * 100.0,
                    cpu.system * 100.0,
                    cpu.interrupt * 100.0,
                    cpu.idle * 100.0
                ),
            );
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }

    match sys.cpu_temp() {
        Ok(cpu_temp) => {
            system_status.insert(String::from("cpu_temp"), format!("{}", cpu_temp));
        }
        Err(x) => println!("\nCPU temp: {}", x),
    }

    system_status
}
