fn main() {
    let hello = "안녕하세요";

    for b in hello.bytes() {
        println!("{:b} {:o} {:x}", b, b, b);
    }
}
