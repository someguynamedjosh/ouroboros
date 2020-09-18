use ouroboros::self_referencing;

#[self_referencing]
struct Test {

}

fn main() {
    test();
}
