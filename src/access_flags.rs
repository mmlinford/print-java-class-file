use crate::primitives::{print_u2, U2};
use std::io::Read;

macro_rules! impl_access_flags {
    {
        $([variant: $variant:ident, flag_name: $flag_name:literal, value: $value:literal],)+
    } => {
        pub enum AccessFlags {
            $($variant,)+
        }

        impl AccessFlags {

            const ALL: &'static [AccessFlags] = &[$(Self::$variant,)+];

            pub fn flag_name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $flag_name,)+
                }
            }

            pub fn value(&self) -> U2 {
                match self {
                    $(Self::$variant => $value,)+
                }
            }

        }
    };
}

impl_access_flags! {
    [variant: Public, flag_name: "ACC_PUBLIC", value: 0x0001],
    [variant: Final, flag_name: "ACC_FINAL", value: 0x0010],
    [variant: Super, flag_name: "ACC_SUPER", value: 0x0020],
    [variant: Interface, flag_name: "ACC_INTERFACE", value: 0x0200],
    [variant: Abstract, flag_name: "ACC_ABSTRACT", value: 0x0400],
    [variant: Synthetic, flag_name: "ACC_SYNTHETIC", value: 0x1000],
    [variant: Annotation, flag_name: "ACC_ANNOTATION", value: 0x2000],
    [variant: Enum, flag_name: "ACC_ENUM", value: 0x4000],
    [variant: Module, flag_name: "ACC_MODULE", value: 0x8000],
}
