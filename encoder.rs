// Run: rustc encode.rs && ./encode
fn main() {
    let key: u8 = 0x5A;
       
// _____________Just change this part and touch nothing else_________________//    
    let url = b"example.com";   // <- change the URL inside double quotes ("").
// _________________________________________________//   
    
    let out: Vec<String> = url.iter().map(|b| format!("0x{:02X}", b ^ key)).collect();
    println!("const URL_KEY: u8 = 0x{:02X};", key);
    println!("const URL_CIPHER: [u8; {}] = [", url.len());
    for chunk in out.chunks(12) {
        println!("    {},", chunk.join(", "));
    }
    println!("];");
}
