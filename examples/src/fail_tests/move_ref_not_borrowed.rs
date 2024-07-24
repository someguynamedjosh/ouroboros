use ouroboros::self_referencing;

#[self_referencing]
struct CopyRef {
    data: String,
    #[borrows(data)]
    #[covariant]
    ref1: Option<&'this str>,
    other: String,
    #[borrows(other)]
    #[covariant]
    ref2: Option<&'this str>,
}

fn main() {
    let mut s = CopyRefBuilder {
        data: "test".to_string(),
        ref1_builder: |_| None,
        other: "other".to_string(),
        ref2_builder: |_| None,
    }
    .build();

    s.with_mut(|f| {
        *f.ref1 = Some(f.other);
    });

    drop(s);
}
