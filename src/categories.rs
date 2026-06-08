use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Categories: u32 {
        const COMPUTE    = 0b0000_0001;
        const MEMORY     = 0b0000_0010;
        const FILESYSTEM = 0b0000_0100;
        const REGISTRY   = 0b0000_1000;
        const WINAPI     = 0b0001_0000;
        const NETWORK    = 0b0010_0000;
        const CRYPTO     = 0b0100_0000;
        const COM        = 0b1000_0000;
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
        #[cfg(feature = "cat-com")]
        {
            cats |= Self::COM;
        }
        cats
    }
}
