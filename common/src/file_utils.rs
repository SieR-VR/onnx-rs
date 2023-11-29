use std::{error::Error, fs::File, io::Read, path::Path};
use prost::Message;

pub fn load_proto_from_path<P: AsRef<Path>, T: Message + Default>(path: P) -> Result<T, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let proto = T::decode::<&[u8]>(buffer.as_slice())?;
    Ok(proto)
}
