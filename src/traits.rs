use result::GResult;
use std::io::Write;
use types::Types;
use weedle::{
    types::*,
    common::Identifier
};

pub trait WriteBindings {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()>;
}

// Trait to check if the type is defined type in JS Environment. Checks on the custom Types
// if the `self` is an identifier
pub trait IsDefined {
    fn is_defined(&self, custom: &Types) -> bool;
}

// Convert a rust keyword into a safe name by appending _
pub trait ToSafeName {
    fn to_safe_name(&self) -> String;
}

const KEYWORDS: [&'static str; 4] = ["type", "self", "as", "loop"];

impl ToSafeName for String {
    fn to_safe_name(&self) -> String {
        let mut string = self.to_string();
        if KEYWORDS.contains(&string.as_ref())  {
            string.push_str("_");
        }
        string
    }
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

impl WriteBindings for Type {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        match *self {
            Type::Single(ref single) => single.write_bindings(buf),
            Type::Union(_) => unreachable!()
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

impl WriteBindings for SingleType {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        match *self {
            SingleType::Any(_) => unreachable!(),
            SingleType::Promise(_) => unreachable!(),
            SingleType::Integer(ref int) => int.write_bindings(buf)?,
            SingleType::FloatingPoint(ref float) => float.write_bindings(buf)?,
            SingleType::Boolean(_) => write!(buf, "bool")?,
            SingleType::Byte(_) => write!(buf, "i8")?,
            SingleType::Octet(_) => write!(buf, "u8")?,
            SingleType::ByteString(_) => unreachable!(),
            SingleType::DOMString(_) => write!(buf, "String")?,
            SingleType::USVString(_) => write!(buf, "String")?,
            SingleType::Sequence(_) => unreachable!(),
            SingleType::Object(_) => unreachable!(),
            SingleType::Symbol(_) => write!(buf, "JsValue")?,
            SingleType::Error(_) => write!(buf, "JsValue")?,
            SingleType::ArrayBuffer(_) => unreachable!(),
            SingleType::DataView(_) => unreachable!(),
            SingleType::Int8Array(_) => unreachable!(),
            SingleType::Int16Array(_) => unreachable!(),
            SingleType::Int32Array(_) => unreachable!(),
            SingleType::Uint8Array(_) => unreachable!(),
            SingleType::Uint16Array(_) => unreachable!(),
            SingleType::Uint32Array(_) => unreachable!(),
            SingleType::Uint8ClampedArray(_) => unreachable!(),
            SingleType::Float32Array(_) => unreachable!(),
            SingleType::Float64Array(_) => unreachable!(),
            SingleType::FrozenArrayType(_) => unreachable!(),
            SingleType::RecordType(_) => unreachable!(),
            SingleType::Identifier(ref identifier) => identifier.write_bindings(buf)?,
        };
        Ok(())
    }
}

impl<T: IsDefined> IsDefined for MayBeNull<T> {
    fn is_defined(&self, custom: &Types) -> bool {
        if self.q_mark.is_some() {
            return false;
        }
        self.type_.is_defined(custom)
    }
}

impl<I: WriteBindings> WriteBindings for MayBeNull<I> {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        // currently does not support optional
        self.type_.write_bindings(buf)
    }
}

impl IsDefined for Identifier {
    fn is_defined(&self, custom: &Types) -> bool {
        custom.has(&self.name)
    }
}

impl WriteBindings for Identifier {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        write!(buf, "{}", self.name)?;
        Ok(())
    }
}

impl WriteBindings for IntegerType {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        match *self {
            IntegerType::Short(ref short) => short.write_bindings(buf),
            IntegerType::LongLong(ref longlong) => longlong.write_bindings(buf),
            IntegerType::Long(ref long) => long.write_bindings(buf),
        }
    }
}

impl WriteBindings for ShortType {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        if self.unsigned.is_some() {
            write!(buf, "u16")?;
        } else {
            write!(buf, "i16")?;
        }
        Ok(())
    }
}

impl WriteBindings for LongLongType {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        if self.unsigned.is_some() {
            write!(buf, "u64")?;
        } else {
            write!(buf, "i64")?;
        }
        Ok(())
    }
}

impl WriteBindings for LongType {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        if self.unsigned.is_some() {
            write!(buf, "u32")?;
        } else {
            write!(buf, "i32")?;
        }
        Ok(())
    }
}

impl WriteBindings for FloatingPointType {
    fn write_bindings<T: Write>(&self, buf: &mut T) -> GResult<()> {
        match *self {
            FloatingPointType::Float(_) => write!(buf, "f32")?,
            FloatingPointType::Double(_) => write!(buf, "f64")?
        };
        Ok(())
    }
}
