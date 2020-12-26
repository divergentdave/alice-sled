const RANDOM_BYTES: &[u8] = include_bytes!("../../random.bin");

fn shift_bytes_by(shift: usize) -> sled::IVec {
    let mut buf = Vec::with_capacity(RANDOM_BYTES.len());
    for i in 0..RANDOM_BYTES.len() {
        buf.push(RANDOM_BYTES[(i + shift) % RANDOM_BYTES.len()]);
    }
    buf.into()
}

fn main() -> anyhow::Result<()> {
    let db =
        sled::Config::new().segment_size(256).path("workload_dir").open()?;
    for i in 0..10 {
        let key = shift_bytes_by(i);
        let value = shift_bytes_by(i + 10);
        db.insert(key, value)?;
    }
    db.insert(b"large value", vec![b'A'; 1024 * 1024])?;
    db.flush()?;
    println!("Flushed");
    Ok(())
}
