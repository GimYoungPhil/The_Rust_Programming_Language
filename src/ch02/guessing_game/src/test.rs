fn main() {
  let a: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4];
  let b: [f32; 4] = [0.5, 0.6, 0.7, 0.8];

  let c: &[f32] = &a;
  let d: &[f32; 4] = &b;
}
