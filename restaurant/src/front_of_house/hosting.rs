pub fn add_to_waitlist() {
    seat_at_table(); // <-- calling a private function
}

fn seat_at_table() {} // private, but in the same crate
