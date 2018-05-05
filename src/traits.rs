use result::GResult;
use std::io::Write;
use types::Types;
use weedle::{
    types::{Type, SingleType, MayBeNull},
    common::Identifier
};

pub trait Scrape: Sized {
    type From;

    fn scrape(from: &Self::From) -> Self;
}

pub trait WriteBindings {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()>;
}

// Trait to check if the type is defined type in JS Environment. Checks on the custom Types
// if the `self` is an identifier
pub trait IsDefined {
    fn is_defined(&self, custom: &Types) -> bool;
}

impl IsDefined for Type {
    fn is_defined(&self, custom: &Types) -> bool {
        match *self {
            Type::Single(ref single) => single.is_defined(custom),
            // Don't know how to map union types to JS Environment
            Type::Union(_) => false
        }
    }
}

impl IsDefined for SingleType {
    fn is_defined(&self, custom: &Types) -> bool {
        match *self {
            // How to handle `any` with `JSValue`?
            SingleType::Any(_) => false,
            // Currently bindgen does not support Promises
            SingleType::Promise(_) => false,
            SingleType::Integer(_) => true,
            SingleType::FloatingPoint(_) => true,
            SingleType::Boolean(_) => true,
            SingleType::Byte(_) => true,
            SingleType::Octet(_) => true,
            // No support for Vec<T>
            SingleType::ByteString(_) => false,
            SingleType::DOMString(_) => true,
            SingleType::USVString(_) => true,
            // No support for Vec<T>
            SingleType::Sequence(_) => false,
            // Handle with a Map? No Map support.
            SingleType::Object(_) => false,
            SingleType::Symbol(_) => true,
            SingleType::Error(_) => true,
            // No support for Vec<T>
            SingleType::ArrayBuffer(_) => false,
            SingleType::DataView(_) => false,
            SingleType::Int8Array(_) => false,
            SingleType::Int16Array(_) => false,
            SingleType::Int32Array(_) => false,
            SingleType::Uint8Array(_) => false,
            SingleType::Uint16Array(_) => false,
            SingleType::Uint32Array(_) => false,
            SingleType::Uint8ClampedArray(_) => false,
            SingleType::Float32Array(_) => false,
            SingleType::Float64Array(_) => false,
            SingleType::FrozenArrayType(_) => false,
            // No Map support
            SingleType::RecordType(_) => false,
            SingleType::Identifier(ref identifier) => identifier.is_defined(custom),
        }
    }
}

impl<T: IsDefined> IsDefined for MayBeNull<T> {
    fn is_defined(&self, custom: &Types) -> bool {
        self.type_.is_defined(custom)
    }
}

impl IsDefined for Identifier {
    fn is_defined(&self, custom: &Types) -> bool {
        custom.has(&self.name)
    }
}
