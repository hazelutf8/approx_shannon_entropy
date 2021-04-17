use std::io::Read;
use approx_shannon_entropy::shannon_entropy;

fn main() -> std::io::Result<()> {
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines)?;
    println!("Input from stdin: {:?}", lines);

    let entropy: f32 = shannon_entropy(lines.as_bytes());
    println!("Shannon Entropy (approximate bits per byte): {}", entropy);
    Ok(())
}
