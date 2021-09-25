use tornarec::NintendoDS;

fn run() {
    let mut nintendo_ds = NintendoDS::new();
    nintendo_ds.load_file_to_ram("./tests/input_files/hello_world.nds");
}

fn main() {
    run();
}
