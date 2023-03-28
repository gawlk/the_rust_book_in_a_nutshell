pub fn main() {
    if (false) {
        panic!("crash and burn");
    }

    if (false) {
        let v = vec![1, 2, 3];
        #[allow(clippy::unnecessary_operation)]
        v[99];
    }
}
