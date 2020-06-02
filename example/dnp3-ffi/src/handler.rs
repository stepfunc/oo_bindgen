use crate::ffi;
use dnp3::app::enums::QualifierCode;
use dnp3::app::flags::Flags;
use dnp3::app::header::{ResponseFunction, ResponseHeader};
use dnp3::app::measurement::*;
use dnp3::app::types::DoubleBit;
use dnp3::app::variations::Variation;
use dnp3::master::handle::{HeaderInfo, ReadHandler};

unsafe impl Send for ffi::ReadHandler {}
unsafe impl Sync for ffi::ReadHandler {}

pub struct ReadHandlerAdapter {
    native_cb: ffi::ReadHandler,
}

impl ReadHandlerAdapter {
    pub fn new(native_cb: ffi::ReadHandler) -> Self {
        Self { native_cb }
    }
}

impl ReadHandler for ReadHandlerAdapter {
    fn begin_fragment(&mut self, header: ResponseHeader) {
        if let Some(cb) = self.native_cb.begin_fragment {
            let header = header.into();
            (cb)(header, self.native_cb.arg);
        }
    }

    fn end_fragment(&mut self, header: ResponseHeader) {
        if let Some(cb) = self.native_cb.end_fragment {
            let header = header.into();
            (cb)(header, self.native_cb.arg);
        }
    }

    fn handle_binary(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (Binary, u16)>) {
        if let Some(cb) = self.native_cb.handle_binary {
            let info = info.into();
            let mut iterator = BinaryIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_double_bit_binary(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (DoubleBitBinary, u16)>) {
        if let Some(cb) = self.native_cb.handle_double_bit_binary {
            let info = info.into();
            let mut iterator = DoubleBitBinaryIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_binary_output_status(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (BinaryOutputStatus, u16)>) {
        if let Some(cb) = self.native_cb.handle_binary_output_status {
            let info = info.into();
            let mut iterator = BinaryOutputStatusIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_counter(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (Counter, u16)>) {
        if let Some(cb) = self.native_cb.handle_counter {
            let info = info.into();
            let mut iterator = CounterIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_frozen_counter(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (FrozenCounter, u16)>) {
        if let Some(cb) = self.native_cb.handle_frozen_counter {
            let info = info.into();
            let mut iterator = FrozenCounterIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_analog(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (Analog, u16)>) {
        if let Some(cb) = self.native_cb.handle_analog {
            let info = info.into();
            let mut iterator = AnalogIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_analog_output_status(&mut self, info: HeaderInfo, iter: &mut dyn Iterator<Item = (AnalogOutputStatus, u16)>) {
        if let Some(cb) = self.native_cb.handle_analog_output_status {
            let info = info.into();
            let mut iterator = AnalogOutputStatusIterator::new(iter);
            (cb)(info, &mut iterator as *mut _, self.native_cb.arg);
        }
    }

    fn handle_octet_string<'a>(
        &mut self,
        _info: HeaderInfo,
        _iter: &mut dyn Iterator<Item = (dnp3::app::parse::bytes::Bytes<'a>, u16)>,
    ) {
        // TODO: implement this
    }
}

impl From<ResponseHeader> for ffi::ResponseHeader {
    fn from(header: ResponseHeader) -> ffi::ResponseHeader {
        ffi::ResponseHeader {
            control: ffi::Control {
                fir: header.control.fir,
                fin: header.control.fin,
                con: header.control.con,
                uns: header.control.uns,
                seq: header.control.seq.value(),
            },
            func: match header.function {
                ResponseFunction::Response => ffi::ResponseFunction::Response,
                ResponseFunction::UnsolicitedResponse => ffi::ResponseFunction::UnsolicitedResponse,
            },
            iin: ffi::IIN {
                iin1: ffi::IIN1 { value: header.iin.iin1.value },
                iin2: ffi::IIN2 { value: header.iin.iin2.value },
            },
        }
    }
}

impl From<HeaderInfo> for ffi::HeaderInfo {
    fn from(info: HeaderInfo) -> ffi::HeaderInfo {
        ffi::HeaderInfo {
            variation: match info.variation {
                Variation::Group1Var0 => ffi::Variation::Group1Var0,
                Variation::Group1Var1 => ffi::Variation::Group1Var1,
                Variation::Group1Var2 => ffi::Variation::Group1Var2,
                Variation::Group2Var0 => ffi::Variation::Group2Var0,
                Variation::Group2Var1 => ffi::Variation::Group2Var1,
                Variation::Group2Var2 => ffi::Variation::Group2Var2,
                Variation::Group2Var3 => ffi::Variation::Group2Var3,
                Variation::Group3Var0 => ffi::Variation::Group3Var0,
                Variation::Group3Var1 => ffi::Variation::Group3Var1,
                Variation::Group3Var2 => ffi::Variation::Group3Var2,
                Variation::Group4Var0 => ffi::Variation::Group4Var0,
                Variation::Group4Var1 => ffi::Variation::Group4Var1,
                Variation::Group4Var2 => ffi::Variation::Group4Var2,
                Variation::Group4Var3 => ffi::Variation::Group4Var3,
                Variation::Group10Var0 => ffi::Variation::Group10Var0,
                Variation::Group10Var1 => ffi::Variation::Group10Var1,
                Variation::Group10Var2 => ffi::Variation::Group10Var2,
                Variation::Group11Var0 => ffi::Variation::Group11Var0,
                Variation::Group11Var1 => ffi::Variation::Group11Var1,
                Variation::Group11Var2 => ffi::Variation::Group11Var2,
                Variation::Group12Var0 => ffi::Variation::Group12Var0,
                Variation::Group12Var1 => ffi::Variation::Group12Var1,
                Variation::Group13Var1 => ffi::Variation::Group13Var1,
                Variation::Group13Var2 => ffi::Variation::Group13Var2,
                Variation::Group20Var0 => ffi::Variation::Group20Var0,
                Variation::Group20Var1 => ffi::Variation::Group20Var1,
                Variation::Group20Var2 => ffi::Variation::Group20Var2,
                Variation::Group20Var5 => ffi::Variation::Group20Var5,
                Variation::Group20Var6 => ffi::Variation::Group20Var6,
                Variation::Group21Var0 => ffi::Variation::Group21Var0,
                Variation::Group21Var1 => ffi::Variation::Group21Var1,
                Variation::Group21Var2 => ffi::Variation::Group21Var2,
                Variation::Group21Var5 => ffi::Variation::Group21Var5,
                Variation::Group21Var6 => ffi::Variation::Group21Var6,
                Variation::Group21Var9 => ffi::Variation::Group21Var9,
                Variation::Group21Var10 => ffi::Variation::Group21Var10,
                Variation::Group22Var0 => ffi::Variation::Group22Var0,
                Variation::Group22Var1 => ffi::Variation::Group22Var1,
                Variation::Group22Var2 => ffi::Variation::Group22Var2,
                Variation::Group22Var5 => ffi::Variation::Group22Var5,
                Variation::Group22Var6 => ffi::Variation::Group22Var6,
                Variation::Group23Var0 => ffi::Variation::Group23Var0,
                Variation::Group23Var1 => ffi::Variation::Group23Var1,
                Variation::Group23Var2 => ffi::Variation::Group23Var2,
                Variation::Group23Var5 => ffi::Variation::Group23Var5,
                Variation::Group23Var6 => ffi::Variation::Group23Var6,
                Variation::Group30Var0 => ffi::Variation::Group30Var0,
                Variation::Group30Var1 => ffi::Variation::Group30Var1,
                Variation::Group30Var2 => ffi::Variation::Group30Var2,
                Variation::Group30Var3 => ffi::Variation::Group30Var3,
                Variation::Group30Var4 => ffi::Variation::Group30Var4,
                Variation::Group30Var5 => ffi::Variation::Group30Var5,
                Variation::Group30Var6 => ffi::Variation::Group30Var6,
                Variation::Group32Var0 => ffi::Variation::Group32Var0,
                Variation::Group32Var1 => ffi::Variation::Group32Var1,
                Variation::Group32Var2 => ffi::Variation::Group32Var2,
                Variation::Group32Var3 => ffi::Variation::Group32Var3,
                Variation::Group32Var4 => ffi::Variation::Group32Var4,
                Variation::Group32Var5 => ffi::Variation::Group32Var5,
                Variation::Group32Var6 => ffi::Variation::Group32Var6,
                Variation::Group32Var7 => ffi::Variation::Group32Var7,
                Variation::Group32Var8 => ffi::Variation::Group32Var8,
                Variation::Group40Var0 => ffi::Variation::Group40Var0,
                Variation::Group40Var1 => ffi::Variation::Group40Var1,
                Variation::Group40Var2 => ffi::Variation::Group40Var2,
                Variation::Group40Var3 => ffi::Variation::Group40Var3,
                Variation::Group40Var4 => ffi::Variation::Group40Var4,
                Variation::Group41Var0 => ffi::Variation::Group41Var0,
                Variation::Group41Var1 => ffi::Variation::Group41Var1,
                Variation::Group41Var2 => ffi::Variation::Group41Var2,
                Variation::Group41Var3 => ffi::Variation::Group41Var3,
                Variation::Group41Var4 => ffi::Variation::Group41Var4,
                Variation::Group42Var0 => ffi::Variation::Group42Var0,
                Variation::Group42Var1 => ffi::Variation::Group42Var1,
                Variation::Group42Var2 => ffi::Variation::Group42Var2,
                Variation::Group42Var3 => ffi::Variation::Group42Var3,
                Variation::Group42Var4 => ffi::Variation::Group42Var4,
                Variation::Group42Var5 => ffi::Variation::Group42Var5,
                Variation::Group42Var6 => ffi::Variation::Group42Var6,
                Variation::Group42Var7 => ffi::Variation::Group42Var7,
                Variation::Group42Var8 => ffi::Variation::Group42Var8,
                Variation::Group43Var1 => ffi::Variation::Group43Var1,
                Variation::Group43Var2 => ffi::Variation::Group43Var2,
                Variation::Group43Var3 => ffi::Variation::Group43Var3,
                Variation::Group43Var4 => ffi::Variation::Group43Var4,
                Variation::Group43Var5 => ffi::Variation::Group43Var5,
                Variation::Group43Var6 => ffi::Variation::Group43Var6,
                Variation::Group43Var7 => ffi::Variation::Group43Var7,
                Variation::Group43Var8 => ffi::Variation::Group43Var8,
                Variation::Group50Var1 => ffi::Variation::Group50Var1,
                Variation::Group50Var3 => ffi::Variation::Group50Var3,
                Variation::Group50Var4 => ffi::Variation::Group50Var4,
                Variation::Group51Var1 => ffi::Variation::Group51Var1,
                Variation::Group51Var2 => ffi::Variation::Group51Var2,
                Variation::Group52Var1 => ffi::Variation::Group52Var1,
                Variation::Group52Var2 => ffi::Variation::Group52Var2,
                Variation::Group60Var1 => ffi::Variation::Group60Var1,
                Variation::Group60Var2 => ffi::Variation::Group60Var2,
                Variation::Group60Var3 => ffi::Variation::Group60Var3,
                Variation::Group60Var4 => ffi::Variation::Group60Var4,
                Variation::Group80Var1 => ffi::Variation::Group80Var1,
                Variation::Group110(_) => ffi::Variation::Group110,
                Variation::Group111(_) => ffi::Variation::Group111,
                Variation::Group112(_) => ffi::Variation::Group112,
                Variation::Group113(_) => ffi::Variation::Group113,
            },
            qualifier: match info.qualifier {
                QualifierCode::Range8 => ffi::QualifierCode::Range8,
                QualifierCode::Range16 => ffi::QualifierCode::Range16,
                QualifierCode::AllObjects => ffi::QualifierCode::AllObjects,
                QualifierCode::Count8 => ffi::QualifierCode::Count8,
                QualifierCode::Count16 => ffi::QualifierCode::Count16,
                QualifierCode::CountAndPrefix8 => ffi::QualifierCode::CountAndPrefix8,
                QualifierCode::CountAndPrefix16 => ffi::QualifierCode::CountAndPrefix16,
                QualifierCode::FreeFormat16 => ffi::QualifierCode::FreeFormat16,
            },
        }
    }
}

macro_rules! implement_iterator {
    ($it_name:ident, $ffi_func_name:ident, $lib_type:ty, $ffi_type:ty) => {
        pub struct $it_name<'a> {
            inner: &'a mut dyn Iterator<Item = ($lib_type, u16)>,
            next: Option<$ffi_type>,
        }
        
        impl<'a> $it_name<'a> {
            fn new(inner: &'a mut dyn Iterator<Item = ($lib_type, u16)>) -> Self {
                Self { 
                    inner,
                    next: None,
                }
            }
        
            fn next(&mut self) {
                self.next = self.inner.next().map(|(value, idx)| {
                    <$ffi_type>::new(
                        idx,
                        value,
                    )
                })
            }
        }
        
        pub unsafe fn $ffi_func_name(it: *mut $it_name) -> *const $ffi_type {
            let it = it.as_mut();
            match it {
                Some(it) => {
                    it.next();
                    match &it.next {
                        Some(value) => value as *const _,
                        None => std::ptr::null(),
                    }
                },
                None => std::ptr::null(),
            }
        }
    }
}

implement_iterator!(BinaryIterator, binary_next, Binary, ffi::Binary);
implement_iterator!(DoubleBitBinaryIterator, doublebitbinary_next, DoubleBitBinary, ffi::DoubleBitBinary);
implement_iterator!(BinaryOutputStatusIterator, binaryoutputstatus_next, BinaryOutputStatus, ffi::BinaryOutputStatus);
implement_iterator!(CounterIterator, counter_next, Counter, ffi::Counter);
implement_iterator!(FrozenCounterIterator, frozencounter_next, FrozenCounter, ffi::FrozenCounter);
implement_iterator!(AnalogIterator, analog_next, Analog, ffi::Analog);
implement_iterator!(AnalogOutputStatusIterator, analogoutputstatus_next, AnalogOutputStatus, ffi::AnalogOutputStatus);

impl ffi::Binary {
    fn new(idx: u16, value: Binary) -> Self {
        Self {
            index: idx,
            value: value.value,
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl ffi::DoubleBitBinary {
    fn new(idx: u16, value: DoubleBitBinary) -> Self {
        Self {
            index: idx,
            value: match value.value {
                DoubleBit::Intermediate => ffi::DoubleBit::Intermediate,
                DoubleBit::DeterminedOff => ffi::DoubleBit::DeterminedOff,
                DoubleBit::DeterminedOn => ffi::DoubleBit::DeterminedOn,
                DoubleBit::Indeterminate => ffi::DoubleBit::Indeterminate,
            },
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl ffi::BinaryOutputStatus {
    fn new(idx: u16, value: BinaryOutputStatus) -> Self {
        Self {
            index: idx,
            value: value.value,
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl ffi::Counter {
    fn new(idx: u16, value: Counter) -> Self {
        Self {
            index: idx,
            value: value.value,
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl ffi::FrozenCounter {
    fn new(idx: u16, value: FrozenCounter) -> Self {
        Self {
            index: idx,
            value: value.value,
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl ffi::Analog {
    fn new(idx: u16, value: Analog) -> Self {
        Self {
            index: idx,
            value: value.value,
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl ffi::AnalogOutputStatus {
    fn new(idx: u16, value: AnalogOutputStatus) -> Self {
        Self {
            index: idx,
            value: value.value,
            flags: value.flags.into(),
            time: value.time.into(),
        }
    }
}

impl From<Flags> for ffi::Flags {
    fn from(flags: Flags) -> ffi::Flags {
        ffi::Flags {
            value: flags.value
        }
    }
}

impl From<Time> for ffi::Time {
    fn from(time: Time) -> ffi::Time {
        ffi::Time {
            value: match time {
                Time::Synchronized(value) => value.raw_value(),
                Time::NotSynchronized(value) => value.raw_value(),
                Time::Invalid => 0,
            },
            quality: match time {
                Time::Synchronized(_) => ffi::TimeQuality::Synchronized,
                Time::NotSynchronized(_) => ffi::TimeQuality::NotSynchronized,
                Time::Invalid => ffi::TimeQuality::Invalid,
            },
        }
    }
}
