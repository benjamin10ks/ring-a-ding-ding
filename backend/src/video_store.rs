pub trait VideoStore {
    fn save_clip(&self, id: &str, data: &[u8]) -> std::io::Result<()>;
    fn load_clip(&self, id: &str) -> std::io::Result<Vec<u8>>;
    fn delete_clip(&self, id: &str) -> std::io::Result<()>;
}

pub struct DiskVideoStore {
    root: std::path::PathBuf,
}

impl DiskVideoStore {
    pub fn new(root: impl Into<std::path::PathBuf>) -> Result<Self, std::io::Error> {
        Ok(DiskVideoStore { root: root.into() })
    }
}

impl VideoStore for DiskVideoStore {
    fn save_clip(&self, id: &str, data: &[u8]) -> std::io::Result<()> {
        std::fs::write(self.root.join(id), data)
    }
    fn load_clip(&self, id: &str) -> std::io::Result<Vec<u8>> {
        std::fs::read(self.root.join(id))
    }
    fn delete_clip(&self, id: &str) -> std::io::Result<()> {
        std::fs::remove_file(self.root.join(id))
    }
}
