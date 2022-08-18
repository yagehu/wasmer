mod bindings;

pub use bindings::*;

use std::mem::MaybeUninit;
use wasmer_types::ValueType;

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Errno {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Filetype {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Eventtype {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Rights {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Eventrwflags {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Subclockflags {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Clockid {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Fdflags {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Preopentype {
    #[inline]
    fn zero_padding_bytes(&self, _bytes: &mut [MaybeUninit<u8>]) {}
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl ValueType for wasi_snapshot0::Fdstat {
    fn zero_padding_bytes(&self, bytes: &mut [MaybeUninit<u8>]) {
        macro_rules! field {
            ($($f:tt)*) => {
                &self.$($f)* as *const _ as usize - self as *const _ as usize
            };
        }
        macro_rules! field_end {
            ($($f:tt)*) => {
                field!($($f)*) + std::mem::size_of_val(&self.$($f)*)
            };
        }
        macro_rules! zero {
            ($start:expr, $end:expr) => {
                for i in $start..$end {
                    bytes[i] = MaybeUninit::new(0);
                }
            };
        }

        self.fs_filetype
            .zero_padding_bytes(&mut bytes[field!(fs_filetype)..field_end!(fs_filetype)]);
        zero!(field_end!(fs_filetype), field!(fs_flags));

        self.fs_flags
            .zero_padding_bytes(&mut bytes[field!(fs_flags)..field_end!(fs_flags)]);
        zero!(field_end!(fs_flags), field!(fs_rights_base));

        self.fs_rights_base
            .zero_padding_bytes(&mut bytes[field!(fs_rights_base)..field_end!(fs_rights_base)]);
        zero!(
            field_end!(fs_rights_inheriting),
            std::mem::size_of_val(self)
        );
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Errno {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self as i32
    }
    fn from_native(n: Self::Native) -> Self {
        match n {
            0 => Self::Success,
            1 => Self::Toobig,
            2 => Self::Acces,
            3 => Self::Addrinuse,
            4 => Self::Addrnotavail,
            5 => Self::Afnosupport,
            6 => Self::Again,
            7 => Self::Already,
            8 => Self::Badf,
            9 => Self::Badmsg,
            10 => Self::Busy,
            11 => Self::Canceled,
            12 => Self::Child,
            13 => Self::Connaborted,
            14 => Self::Connrefused,
            15 => Self::Connreset,
            16 => Self::Deadlk,
            17 => Self::Destaddrreq,
            18 => Self::Dom,
            19 => Self::Dquot,
            20 => Self::Exist,
            21 => Self::Fault,
            22 => Self::Fbig,
            23 => Self::Hostunreach,
            24 => Self::Idrm,
            25 => Self::Ilseq,
            26 => Self::Inprogress,
            27 => Self::Intr,
            28 => Self::Inval,
            29 => Self::Io,
            30 => Self::Isconn,
            31 => Self::Isdir,
            32 => Self::Loop,
            33 => Self::Mfile,
            34 => Self::Mlink,
            35 => Self::Msgsize,
            36 => Self::Multihop,
            37 => Self::Nametoolong,
            38 => Self::Netdown,
            39 => Self::Netreset,
            40 => Self::Netunreach,
            41 => Self::Nfile,
            42 => Self::Nobufs,
            43 => Self::Nodev,
            44 => Self::Noent,
            45 => Self::Noexec,
            46 => Self::Nolck,
            47 => Self::Nolink,
            48 => Self::Nomem,
            49 => Self::Nomsg,
            50 => Self::Noprotoopt,
            51 => Self::Nospc,
            52 => Self::Nosys,
            53 => Self::Notconn,
            54 => Self::Notdir,
            55 => Self::Notempty,
            56 => Self::Notrecoverable,
            57 => Self::Notsock,
            58 => Self::Notsup,
            59 => Self::Notty,
            60 => Self::Nxio,
            61 => Self::Overflow,
            62 => Self::Ownerdead,
            63 => Self::Perm,
            64 => Self::Pipe,
            65 => Self::Proto,
            66 => Self::Protonosupport,
            67 => Self::Prototype,
            68 => Self::Range,
            69 => Self::Rofs,
            70 => Self::Spipe,
            71 => Self::Srch,
            72 => Self::Stale,
            73 => Self::Timedout,
            74 => Self::Txtbsy,
            75 => Self::Xdev,
            76 => Self::Notcapable,
            // TODO: What should we map invalid native values to?
            _ => Self::Inval,
        }
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Advice {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self as i32
    }
    fn from_native(n: Self::Native) -> Self {
        match n {
            0 => Self::Normal,
            1 => Self::Sequential,
            2 => Self::Random,
            3 => Self::Willneed,
            4 => Self::Dontneed,
            5 => Self::Noreuse,
            // TODO: What should we map invalid native values to?
            _ => todo!("Need to decide what to do here…"),
        }
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Rights {
    type Native = i64;

    fn to_native(self) -> Self::Native {
        self.bits() as i64
    }
    fn from_native(n: Self::Native) -> Self {
        Self::from_bits_truncate(n as u64)
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Eventrwflags {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self.bits() as i32
    }
    fn from_native(n: Self::Native) -> Self {
        Self::from_bits_truncate(n as u8)
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Subclockflags {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self.bits() as i32
    }
    fn from_native(n: Self::Native) -> Self {
        Self::from_bits_truncate(n as u8)
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Clockid {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self as i32
    }
    fn from_native(n: Self::Native) -> Self {
        match n {
            0 => Self::Realtime,
            1 => Self::Monotonic,
            2 => Self::ProcessCputimeId,
            3 => Self::ThreadCputimeId,
            // TODO: What should we map invalid native values to?
            _ => todo!("Need to decide what to do here…"),
        }
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Fdflags {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self.bits() as i32
    }
    fn from_native(n: Self::Native) -> Self {
        Self::from_bits_truncate(n as u8)
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}

// TODO: if necessary, must be implemented in wit-bindgen
unsafe impl wit_bindgen_wasmer::wasmer::FromToNativeWasmType for wasi_snapshot0::Preopentype {
    type Native = i32;

    fn to_native(self) -> Self::Native {
        self as i32
    }
    fn from_native(n: Self::Native) -> Self {
        match n {
            0 => Self::Dir,
            // TODO: What should we map invalid native values to?
            _ => todo!("Need to decide what to do here…"),
        }
    }

    #[cfg(feature = "sys")]
    fn is_from_store(&self, _store: &impl wit_bindgen_wasmer::wasmer::AsStoreRef) -> bool {
        // TODO: find correct implementation
        false
    }
}
