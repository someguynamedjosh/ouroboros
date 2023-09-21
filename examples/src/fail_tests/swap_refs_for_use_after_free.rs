use ouroboros::self_referencing;

pub struct PrintStrRef<'a>(&'a str);

impl Drop for PrintStrRef<'_> {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

#[self_referencing]
pub struct Tricky {
    data1: String,
    #[borrows(data1)]
    #[covariant]
    ref1: PrintStrRef<'this>,
    data2: String,
}

fn main() {
    let mut t = Tricky::new(
        "A".to_owned(),
        |a| PrintStrRef(a),
        "B".to_owned(),
    );
    t.with_mut(|fields| {
        *fields.ref1 = PrintStrRef(fields.data2);
    });
    drop(t);
}
