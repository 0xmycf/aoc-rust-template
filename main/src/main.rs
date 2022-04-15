use futures::executor::block_on;

fn main() {
    block_on(main::run()).iter().for_each(|x| println!("{}", x));
}

