#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]

mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[warn(unused)]

#[repr(C)]
pub struct TaggedValue {
    pub tag: bindings::tag_t,
    pub value: bindings::tagged_value_s__bindgen_ty_1,
}

impl TaggedValue {
    pub fn new(size: usize, tag: bindings::tag_t) -> Option<Self> {
        unsafe {
            let tagged_value = bindings::safe_sdmm_malloc(size, tag);
            if tagged_value.value._int_.is_null() {
                return None;
            }

            match tag {
                bindings::tag_enum_TAG_INT => {
                    *tagged_value.value._int_ = 0; 
                }
                bindings::tag_enum_TAG_INT8 => {
                    *tagged_value.value.int8 = 0; 
                }
                bindings::tag_enum_TAG_INT16 => {
                    *tagged_value.value.int16 = 0; 
                }
                bindings::tag_enum_TAG_INT32 => {
                    *tagged_value.value.int32 = 0; 
                }
                bindings::tag_enum_TAG_INT64 => {
                    *tagged_value.value.int64 = 0; 
                }
                bindings::tag_enum_TAG_UINT8 => {
                    *tagged_value.value.uint8 = 0; 
                }
                bindings::tag_enum_TAG_UINT16 => {
                    *tagged_value.value.uint16 = 0; 
                }
                bindings::tag_enum_TAG_UINT32 => {
                    *tagged_value.value.uint32 = 0; 
                }
                bindings::tag_enum_TAG_UINT64 => {
                    *tagged_value.value.uint64 = 0; 
                }
                bindings::tag_enum_TAG_FLOAT32 => {
                    *tagged_value.value.float32 = 0.0;
                }
                bindings::tag_enum_TAG_FLOAT64 => {
                    *tagged_value.value.float64 = 0.0;
                }
                bindings::tag_enum_TAG_STRING => {
                    *tagged_value.value.string = 0;
                }
                _ => {}
            }

            Some(Self {
                tag: tagged_value.tag,
                value: tagged_value.value,
            })
        }
    }

    pub fn set_int(&mut self, value: i32) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_INT {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value._int_ = value;
        }
        Ok(())
    }

    pub fn get_int(&self) -> Option<i32> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT {
                Some(*self.value._int_)
            } else {
                None
            }
        }
    }

    pub fn set_int8(&mut self, value: i8) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_INT8 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int8 = value;
        }
        Ok(())
    }

    pub fn get_int8(&self) -> Option<i8> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT8 {
                Some(*self.value.int8)
            } else {
                None
            }
        }
    }

    pub fn set_int16(&mut self, value: i16) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_INT16 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int16 = value;
        }
        Ok(())
    }

    pub fn get_int16(&self) -> Option<i16> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT16 {
                Some(*self.value.int16)
            } else {
                None
            }
        }
    }

    pub fn set_int32(&mut self, value: i32) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_INT32 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int32 = value;
        }
        Ok(())
    }

    pub fn get_int32(&self) -> Option<i32> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT32 {
                Some(*self.value.int32)
            } else {
                None
            }
        }
    }

    pub fn set_int64(&mut self, value: i64) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_INT64 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.int64 = value;
        }
        Ok(())
    }

    pub fn get_int64(&self) -> Option<i64> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INT64 {
                Some(*self.value.int64)
            } else {
                None
            }
        }
    }

    pub fn set_intptr(&mut self, value: isize) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_INTPTR {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.intptr = value;
        }
        Ok(())
    }

    pub fn get_intptr(&self) -> Option<isize> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_INTPTR {
                Some(*self.value.intptr)
            } else {
                None
            }
        }
    }
    pub fn set_uintptr(&mut self, value: usize) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_UINTPTR {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uintptr = value;
        }
        Ok(())
    }

    pub fn get_uintptr(&self) -> Option<usize> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINTPTR {
                Some(*self.value.uintptr)
            } else {
                None
            }
        }
    }

    pub fn set_uint8(&mut self, value: u8) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_UINT8 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint8 = value;
        }
        Ok(())
    }

    pub fn get_uint8(&self) -> Option<u8> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT8 {
                Some(*self.value.uint8)
            } else {
                None
            }
        }
    }

    pub fn set_uint16(&mut self, value: u16) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_UINT16 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint16 = value;
        }
        Ok(())
    }

    pub fn get_uint16(&self) -> Option<u16> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT16 {
                Some(*self.value.uint16)
            } else {
                None
            }
        }
    }

    pub fn set_uint32(&mut self, value: u32) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_UINT32 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint32 = value;
        }
        Ok(())
    }

    pub fn get_uint32(&self) -> Option<u32> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT32 {
                Some(*self.value.uint32)
            } else {
                None
            }
        }
    }

    pub fn set_uint64(&mut self, value: u64) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_UINT64 {
            return Err("Invalid tag for int value");
        }
        unsafe {
            *self.value.uint64 = value;
        }
        Ok(())
    }

    pub fn get_uint64(&self) -> Option<u64> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_UINT64 {
                Some(*self.value.uint64)
            } else {
                None
            }
        }
    }


    pub fn set_float32(&mut self, value: f32) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_FLOAT32 {
            return Err("Invalid tag for float value");
        }
        unsafe {
            *self.value.float32 = value;
        }
        Ok(())
    }

    pub fn get_float32(&self) -> Option<f32> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_FLOAT32 {
                Some(*self.value.float32)
            } else {
                None
            }
        }
    }

    pub fn set_float64(&mut self, value: f64) -> Result<(), &'static str> {
        if self.tag != bindings::tag_enum_TAG_FLOAT64 {
            return Err("Invalid tag for float value");
        }
        unsafe {
            *self.value.float64 = value;
        }
        Ok(())
    }

    pub fn get_float64(&self) -> Option<f64> {
        unsafe {
            if self.tag == bindings::tag_enum_TAG_FLOAT64 {
                Some(*self.value.float64)
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

}

fn unsafe_test() {
    unsafe {
        // Allocate mem
        let size: usize = 1048;
        let ptr = bindings::sdmm_malloc(size);
        if ptr.is_null() {
            eprintln!("Failed to allocate memory");
            return;
        }

        println!("Allocated {} bytes at {:?}", size, ptr);

        // Store a value
        let value_to_store: i32 = 42;
        let ptr_as_int_ptr = ptr as *mut i32;
        *ptr_as_int_ptr = value_to_store;
        println!("Stored value {} at {:?}", value_to_store, ptr);

        // Retrieve value
        let retrieved_value = *ptr_as_int_ptr;
        println!("Retrieved value {} from {:?}", retrieved_value, ptr);

        // Check usable size
        let usable_size = bindings::sdmm_malloc_usable_size(ptr);
        println!("Usable size of allocated memory: {} bytes", usable_size);

        // Reallocate mem
        let new_size: usize = 2048;
        let new_ptr = bindings::sdmm_realloc(ptr, new_size);
        if new_ptr.is_null() {
            eprintln!("Failed to reallocate memory");
            bindings::sdmm_free(ptr);
            return;
        }

        println!("Reallocated memory to {} bytes at {:?}", new_size, new_ptr);

        // Check value after realloc
        let new_ptr_as_int_ptr = new_ptr as *mut i32;
        let preserved_value = *new_ptr_as_int_ptr;
        println!("Value after reallocation is {} at {:?}", preserved_value, new_ptr);

        // Allocate zero-initialized mem
        let num: usize = 10;
        let element_size: usize = 256;
        let calloc_ptr = bindings::sdmm_calloc(num, element_size);
        if calloc_ptr.is_null() {
            eprintln!("Failed to allocate zero-initialized memory");
            bindings::sdmm_free(new_ptr);
            return;
        }

        println!("Allocated zero-initialized memory for {} elements of size {} at {:?}", num, element_size, calloc_ptr);

        // Verify if zero-initialized
        let calloc_ptr_as_int_ptr = calloc_ptr as *mut i32;
        let zero_value = *calloc_ptr_as_int_ptr;
        println!("Value at zero-initialized memory is {} at {:?}", zero_value, calloc_ptr);

        bindings::sdmm_free(new_ptr);
        bindings::sdmm_free(calloc_ptr);

        println!("Freed allocated memory");
    }
}

fn main() {
    unsafe_test();
    let size: usize = 1048;
    let tag = bindings::tag_enum_TAG_INT;
    let int_container = TaggedValue::new(size, tag);
    if int_container.is_none() {
        eprintln!("Failed to allocate memory");
        return;
    }
    let mut int_container = int_container.unwrap();

    println!("Allocated memory with tag {:?} at {:?}", int_container.tag, unsafe { int_container.value._int_ });

    int_container.set_int(42).expect("Failed to set int value");

    if let Some(value) = int_container.get_int() {
        println!("Stored int value: {}", value);
    } else {
        eprintln!("Failed to retrieve int value");
    }
}