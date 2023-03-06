fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let dose_not_exist = &v[100]; // panic
    let dose_not_exist = v.get(100); // None
}
