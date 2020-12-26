const RANDOM_BYTES: &[u8] = include_bytes!("../../random.bin");

fn shift_bytes_by(shift: usize) -> sled::IVec {
    let mut buf = Vec::with_capacity(RANDOM_BYTES.len());
    for i in 0..RANDOM_BYTES.len() {
        buf.push(RANDOM_BYTES[(i + shift) % RANDOM_BYTES.len()]);
    }
    buf.into()
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let crashed_state_directory = args.next().unwrap();
    let stdout_file = args.next().unwrap();
    let stdout = std::fs::read_to_string(stdout_file).unwrap();
    let db = sled::Config::new()
        .segment_size(256)
        .path(crashed_state_directory)
        .open()?;
    if stdout.contains("Flushed") {
        for i in 0..10 {
            let key = shift_bytes_by(i);
            let value = shift_bytes_by(i + 10);
            assert_eq!(db.get(key)?, Some(value));
        }
        assert_eq!(
            db.get(b"large value")?.as_ref().map(AsRef::as_ref),
            Some(&[b'A'; 1024 * 1024][..]),
        );
    }
    Ok(())
}
