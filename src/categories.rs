use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Categories: u32 {
        const COMPUTE    = 0b000_0001;
        const MEMORY     = 0b000_0010;
        const FILESYSTEM = 0b000_0100;
        const REGISTRY   = 0b000_1000;
        const WINAPI     = 0b001_0000;
        const NETWORK    = 0b010_0000;
        const CRYPTO     = 0b100_0000;
    }
}

impl Categories {
    pub fn available() -> Self {
        let mut cats = Self::empty();
        #[cfg(feature = "cat-compute")]
        {
            cats |= Self::COMPUTE;
        }
        #[cfg(feature = "cat-memory")]
        {
            cats |= Self::MEMORY;
        }
        #[cfg(feature = "cat-filesystem")]
        {
            cats |= Self::FILESYSTEM;
        }
        #[cfg(feature = "cat-registry")]
        {
            cats |= Self::REGISTRY;
        }
        #[cfg(feature = "cat-winapi")]
        {
            cats |= Self::WINAPI;
        }
        #[cfg(feature = "cat-network")]
        {
            cats |= Self::NETWORK;
        }
        #[cfg(feature = "cat-crypto")]
        {
            cats |= Self::CRYPTO;
        }
        cats
    }
}
