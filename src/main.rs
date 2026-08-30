use sysinfo::{System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu = sys.global_cpu_usage();
    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_pct = used as f64 / total as f64 * 100.0;

    println!("Rustito - System Cockpit");
    println!("CPU usage: {:.1}%", cpu);
    println!(
        "Memory: {:.2} GiB used of {:.2} GiB ({:.1}%)",
        used as f64 / 1_073_741_824.0,
        total as f64 / 1_073_741_824.0,
        mem_pct
    );

    for cpu in sys.cpus() {
        println!("  CPU {}: {:.1}%", cpu.name(), cpu.cpu_usage());
    }
}