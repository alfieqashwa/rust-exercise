mod basic;
use basic::{cello, qashwa};

fn main() {
    // basic
    cello::cello_world();

    let name = String::from("Cello");
    cello::cello_name(&name);

    qashwa::qashwa_world();

    let name = "Qashwa".to_string();
    qashwa::qashwa_name(&name);
}
