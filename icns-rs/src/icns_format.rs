const MAGIC: [u8; 4] = [0x69, 0x63, 0x6e, 0x73]; // "icns"

/// ## IcnsDataEntry
/// This file contains both the OSType and the data.
/// The OSType is a 4-byte identifier that tells the OS what the data is.
/// The data is the actual image / whatever data is being stored.
/// Data can be images, masks, metadata, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IcnsDataEntry {
    pub os_type: [u8; 4],
    pub data: Box<[u8]>,
}

impl IcnsDataEntry {
    /// ## New
    /// Creates a new IcnsDataEntry.
    pub fn new(os_type: [u8; 4], data: Box<[u8]>) -> Self {
        Self { os_type, data }
    }

    /// ## Length
    /// This function gets the length of the data when compiled.
    /// The length is 4 bytes (OSType) + 4 bytes (length) + length (data)
    pub fn len(&self) -> u32 {
        (8 + self.data.len()) as u32
    }

    /// ## Building the data
    /// This function compiles the data into a single byte array.
    /// This contains the OSType followed by the length of the data
    /// followed by the data.
    pub fn build(&self) -> Box<[u8]> {
        // Total: 4 bytes (OSType) + 4 bytes (length) + length (data)
        let mut result = Vec::with_capacity(self.len() as usize);

        result.extend_from_slice(&self.os_type);
        result.extend_from_slice(&(8 + self.data.len() as u32).to_be_bytes());
        result.extend_from_slice(&self.data);

        result.into_boxed_slice()
    }
}

/// ## ICNSBuilder
/// This struct holds a list of data that will be compiled into an ICNS file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconFamily {
    pub data: Vec<IcnsDataEntry>,
}

impl IconFamily {
    /// ## New
    /// Creates a new file format
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    /// ## Adding data
    /// This adds a new entry to the file.
    /// Data can be images, masks, metadata, etc.
    pub fn add_data(&mut self, data: IcnsDataEntry) -> &mut Self {
        self.data.push(data);

        self
    }

    /// ## Creating the table of contents
    /// The table of contents is the first entry in the file.
    /// It contains the OSType of each entry and the length of each entry.
    pub fn create_contents_table(&self) -> IcnsDataEntry {
        let mut buffer = Vec::with_capacity(8 * self.data.len()); // Each entry is 8 bytes

        for data in &self.data {
            buffer.extend_from_slice(&data.os_type);
            buffer.extend_from_slice(&((&data).data.len() as u32).to_be_bytes());
        }

        IcnsDataEntry::new(
            [0x54, 0x4F, 0x43, 0x20], // "TOC "
            buffer.into_boxed_slice(),
        )
    }

    /// ## Building the ICNS file
    /// Building the file will create the table of contents
    /// and compile all the data into a single file.
    pub fn build(&self) -> Box<[u8]> {
        // Calculate the total size of the file
        let contents_table = self.create_contents_table();

        // Insert the TOC first
        let mut data = Vec::with_capacity(self.data.len() + 1);
        data.push(contents_table);
        for d in &self.data {
            data.push(d.clone());
        }

        let total_size = data.iter().map(|data| data.len()).sum::<u32>();
        let mut buffer = Vec::with_capacity(MAGIC.len() + 4 + total_size as usize);

        // Add the magic bytes, the total size and the data
        buffer.extend_from_slice(&MAGIC);
        buffer.extend_from_slice(&(total_size as u32).to_be_bytes());
        for data in &data {
            buffer.extend_from_slice(&data.build());
        }

        buffer.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_icns_data_entry() {
        let dummy_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
        let result = vec![
            0x00, 0x00, 0x00, 0x00, // OSType: NUL NUL NUL NUL
            0x00, 0x00, 0x00,
            0x0C, // Length: 12 (4 bytes OSType + 4 bytes length + 4 bytes data)
            0x00, 0x01, 0x02, 0x03, // Data: 00 01 02 03
        ];

        let entry =
            super::IcnsDataEntry::new([0x00, 0x00, 0x00, 0x00], dummy_data.into_boxed_slice());

        assert_eq!(entry.build(), result.into_boxed_slice());
    }
}
