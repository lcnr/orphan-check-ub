use b::B;
use c::C;

fn main() {
    let storage = b::init_storage::<C>(0);
    c::read_storage::<B, C>(storage)
}
