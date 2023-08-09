mod db_loader;

fn main() {
    println!("Hello, world!");
    let database = db_loader::load_database("files");
}


