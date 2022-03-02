use futures::executor::block_on;

fn main() {
    block_on(main::run()).iter().for_each(printme);
}

fn printme<A: std::fmt::Display>(printable: A) {
    println!("{printable}");
}

