use crate::sdmm_mod::bindings::*;

#[repr(C)]
pub struct ValueDescriptor {
    pub tag: bindings::tag_t,
    pub value: bindings::tagged_value_s__bindgen_ty_1,
}

impl ValueDescriptor {
    const DEFAULT_ELEM: usize = 1;
    const DEFAULT_POS: usize = 0;
    pub fn new(tag: bindings::tag_t, num_elements: Option<usize>) -> Option<Self> {
        let num_elements = num_elements.unwrap_or(Self::DEFAULT_ELEM);
        unsafe {
            let vd = bindings::safe_sdmm_malloc(num_elements, tag);
            if vd.value._int_.is_null() {
                return None;
            }

            match tag {
                bindings::tag_enum_TAG_INT => {
                    for index in 0..num_elements {
                        // .add is supposedly able to find ith element ? 
                        *vd.value._int_.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_INT8 => {
                    for index in 0..num_elements {
                        *vd.value.int8.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_INT16 => {
                    for index in 0..num_elements {
                        *vd.value.int16.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_INT32 => {
                    for index in 0..num_elements {
                        *vd.value.int32.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_INT64 => {
                    for index in 0..num_elements {
                        *vd.value.int64.add(index) = 0;
                    } 
                }
                bindings::tag_enum_TAG_UINT8 => {
                    for index in 0..num_elements {
                        *vd.value.uint8.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_UINT16 => {
                    for index in 0..num_elements {
                        *vd.value.uint16.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_UINT32 => {
                    for index in 0..num_elements {
                        *vd.value.uint32.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_UINT64 => {
                    for index in 0..num_elements {
                        *vd.value.uint64.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_INTPTR => {
                    for index in 0..num_elements {
                        *vd.value.intptr.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_UINTPTR => {
                    for index in 0..num_elements {
                        *vd.value.uintptr.add(index) = 0;
                    }
                }
                bindings::tag_enum_TAG_FLOAT32 => {
                    for index in 0..num_elements {
                        *vd.value.float32.add(index) = 0.0;
                    }
                }
                bindings::tag_enum_TAG_FLOAT64 => {
                    for index in 0..num_elements {
                        *vd.value.float64.add(index) = 0.0;
                    }
                }
                bindings::tag_enum_TAG_STRING => {
                    *vd.value.string = 0;
                }
                _ => {}
            }

            Some(Self {
                tag: vd.tag,
                value: vd.value,
            })
        }
    }
    
    pub fn set_int(&mut self, value: i32, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_INT {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value._int_.add(index) = value;
        }
        Ok(())
    }

    pub fn get_int(&self, index: Option<usize>) -> Option<i32> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT {
                Some(*self.value._int_.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_int8(&mut self, value: i8, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_INT8 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int8.add(index) = value;
        }
        Ok(())
    }

    pub fn get_int8(&self, index: Option<usize>) -> Option<i8> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT8 {
                Some(*self.value.int8.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_int16(&mut self, value: i16, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_INT16 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int16.add(index) = value;
        }
        Ok(())
    }

    pub fn get_int16(&self, index: Option<usize>) -> Option<i16> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT16 {
                Some(*self.value.int16.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_int32(&mut self, value: i32, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_INT32 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int32.add(index) = value;
        }
        Ok(())
    }

    pub fn get_int32(&self, index: Option<usize>) -> Option<i32> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT32 {
                Some(*self.value.int32.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_int64(&mut self, value: i64, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_INT64 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int64.add(index) = value;
        }
        Ok(())
    }

    pub fn get_int64(&self, index: Option<usize>) -> Option<i64> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT64 {
                Some(*self.value.int64.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_intptr(&mut self, value: isize, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_INTPTR {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.intptr.add(index) = value;
        }
        Ok(())
    }

    pub fn get_intptr(&self, index: Option<usize>) -> Option<isize> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INTPTR {
                Some(*self.value.intptr.add(index))
            } else {
                None
            }
        }
    }
    pub fn set_uintptr(&mut self, value: usize, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_UINTPTR {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uintptr.add(index) = value;
        }
        Ok(())
    }

    pub fn get_uintptr(&self, index: Option<usize>) -> Option<usize> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINTPTR {
                Some(*self.value.uintptr.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_uint8(&mut self, value: u8, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_UINT8 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint8.add(index) = value;
        }
        Ok(())
    }

    pub fn get_uint8(&self, index: Option<usize>) -> Option<u8> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT8 {
                Some(*self.value.uint8.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_uint16(&mut self, value: u16, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_UINT16 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint16.add(index) = value;
        }
        Ok(())
    }

    pub fn get_uint16(&self, index: Option<usize>) -> Option<u16> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT16 {
                Some(*self.value.uint16.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_uint32(&mut self, value: u32, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_UINT32 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint32.add(index) = value;
        }
        Ok(())
    }

    pub fn get_uint32(&self, index: Option<usize>) -> Option<u32> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT32 {
                Some(*self.value.uint32.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_uint64(&mut self, value: u64, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_UINT64 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint64.add(index) = value;
        }
        Ok(())
    }

    pub fn get_uint64(&self, index: Option<usize>) -> Option<u64> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT64 {
                Some(*self.value.uint64.add(index))
            } else {
                None
            }
        }
    }


    pub fn set_float32(&mut self, value: f32, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_FLOAT32 {
            return Err("Invalid tag for float value");
        }
        unsafe {
            *self.value.float32.add(index) = value;
        }
        Ok(())
    }

    pub fn get_float32(&self, index: Option<usize>) -> Option<f32> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_FLOAT32 {
                Some(*self.value.float32.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_float64(&mut self, value: f64, index: Option<usize>) -> Result<(), &'static str> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        if self.tag != bindings::tag_enum_TAG_FLOAT64 {
            return Err("Invalid tag for float value");
        }
        unsafe {
            *self.value.float64.add(index) = value;
        }
        Ok(())
    }

    pub fn get_float64(&self, index: Option<usize>) -> Option<f64> {
        let index = index.unwrap_or(Self::DEFAULT_POS);
        unsafe {
            if self.tag == bindings::tag_enum_TAG_FLOAT64 {
                Some(*self.value.float64.add(index))
            } else {
                None
            }
        }
    }

    pub fn set_string(&mut self, value: &str) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_STRING {
            return Err("Invalid tag for string value");
        }
            let c_string = std::ffi::CString::new(value).unwrap();
            self.value.string = c_string.into_raw();
        Ok(())
    }

    pub fn get_string(&self) -> Option<String> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_STRING {
                Some(std::ffi::CStr::from_ptr(self.value.string).to_string_lossy().into_owned())
            } else {
                None
            }
        }
    }

    fn to_c_struct(&self) -> bindings::tagged_value_t {
        bindings::tagged_value_t {
            tag: self.tag,
            value: self.value,
        }
    }
    pub fn free(&mut self) {
        unsafe {
            bindings::safe_sdmm_free(self.to_c_struct());
        }
    }
}