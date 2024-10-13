use raidz2::{Disk, Raidz2Config};
use std::io;

fn find_configurations(
    target_storage: f64,
    disk_options: &[Disk],
    range_ratio: f64,
) -> Vec<Raidz2Config> {
    let min_storage = target_storage * (1.0 - range_ratio);
    let max_storage = target_storage * (1.0 + range_ratio);

    disk_options
        .iter()
        .flat_map(|disk| {
            // Testing from 4 to 20 disks
            (4..=20).filter_map(|num_disks| {
                let config = Raidz2Config::new(disk.clone(), num_disks);
                let usable_storage = config.usable_storage_tb();
                if usable_storage >= min_storage && usable_storage <= max_storage {
                    Some(config)
                } else {
                    None
                }
            })
        })
        .collect()
}

fn print_configurations(configurations: &[Raidz2Config]) {
    println!(
        "{:<20} {:<10} {:<20} {:<20} {:<20} {:<20} {:<20}",
        "Disk Size (TB)",
        "# Disks",
        "Usable Storage (TB)",
        "Raw Storage (TB)",
        "Total Cost",
        "Cost per usable TB",
        "Cost per raw TB"
    );
    println!(
        "{:<20} {:<10} {:<20} {:<20} {:<20} {:<20} {:<20}",
        "---------------",
        "-------",
        "-----------------",
        "-----------------",
        "-----------",
        "----------------",
        "--------------"
    );

    for config in configurations {
        println!(
            "{:<20} {:<10} {:<20} {:<20} {:<20} {:<20} {:<20}",
            format!("{:.2}", config.disk.size),
            config.num_disks,
            format!("{:.2}", config.usable_storage_tb()),
            format!("{:.2}", config.raw_storage_tb()),
            format!("{:.2}", config.total_cost()),
            format!("{:.2}", config.total_cost() / config.usable_storage_tb()),
            format!("{:.2}", config.total_cost() / config.raw_storage_tb())
        );
    }
}

fn main() {
    let disks = [
        Disk {
            size: 2.0,
            cost: 1128.0,
        },
        Disk {
            size: 3.0,
            cost: 1502.0,
        },
        Disk {
            size: 4.0,
            cost: 1318.0,
        },
        Disk {
            size: 6.0,
            cost: 2050.0,
        },
        Disk {
            size: 8.0,
            cost: 2328.0,
        },
        Disk {
            size: 12.0,
            cost: 3110.0,
        },
    ];

    let range_percentage = 20.0;

    println!("RAID-Z2 configuration calculator");
    println!("Enter your target usable storage in TB:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let target_storage: f64 = if let Ok(num) = input.trim().parse() {
        num
    } else {
        println!("Invalid input. Please enter a valid number.");
        return;
    };

    println!();

    let mut configurations = find_configurations(target_storage, &disks, range_percentage / 100.0);
    configurations.sort_by(|a, b| a.total_cost().partial_cmp(&b.total_cost()).unwrap());

    if configurations.is_empty() {
        println!(
            "No configurations available within ±{range_percentage}% of {target_storage:.2} TB of usable storage."
        );
    } else {
        println!(
            "Configurations that are within ±{range_percentage}% of {target_storage:.2} TB of usable storage:"
        );
        println!();
        print_configurations(&configurations);
    }
}
