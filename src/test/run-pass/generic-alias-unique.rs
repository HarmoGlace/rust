

fn id<send T>(t: T) -> T { ret t; }

fn main() {
    let expected = ~100;
    let actual = id::<~int>(expected);
    log_full(core::debug, *actual);
    assert (*expected == *actual);
}
