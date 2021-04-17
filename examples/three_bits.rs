use approx_shannon_entropy::shannon_entropy;

fn main() {
    let pattern = [0, 0, 1, 1, 0, 0, 1, 1];
    let entropy: f32 = shannon_entropy(&pattern);
    println!("Shannon Entropy (approximate bits per byte): {}", entropy);
}
