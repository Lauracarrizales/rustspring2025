fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        // Your implementation here
        tracker += 1;
        println!("Counter: {}", tracker);
    };

    update();
    update();
}

fn main() {
    track_changes();
}
