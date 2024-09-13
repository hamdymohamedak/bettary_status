use ak_macros::*;
fn main() {
    akp!("Hello, world!");
    let terminal_value = terminal!("bash", "
        battery_info=$(cat /sys/class/power_supply/BAT0/capacity)
        battery_status=$(cat /sys/class/power_supply/BAT0/status)
        echo \"Battery level: $battery_info%\"
        echo \"Battery status: $battery_status\"
    ");
    println!("{}", terminal_value);
}
