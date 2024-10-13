#[derive(Clone, Debug)]
pub struct Disk {
    pub size: f64,
    pub cost: f64,
}

#[derive(Clone, Debug)]
pub struct Raidz2Config {
    pub disk: Disk,
    pub num_disks: u32,
}

impl Raidz2Config {
    #[must_use]
    pub fn new(disk: Disk, num_disks: u32) -> Self {
        assert!(num_disks >= 4, "RAID-Z2 requires at least 4 disks.");
        Self { disk, num_disks }
    }
    #[must_use]
    pub fn raw_storage_tb(&self) -> f64 {
        self.disk.size * f64::from(self.num_disks)
    }
    #[must_use]
    pub fn usable_storage_tb(&self) -> f64 {
        self.disk.size * f64::from(self.num_disks - 2)
    }
    #[must_use]
    pub fn total_cost(&self) -> f64 {
        self.disk.cost * f64::from(self.num_disks)
    }
}
