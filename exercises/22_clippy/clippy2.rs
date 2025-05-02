fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    #[warn(for_loops_over_fallibles)]
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
