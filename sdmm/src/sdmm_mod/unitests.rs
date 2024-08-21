#[cfg(test)]
mod tests {
    use crate::bindings;
    use crate::ValueDescriptor;
    use crate::sdmm_mod::custom_global_alloc;


    #[test]
    fn test_global_alloc() {
        let skip: bool;

        #[cfg(feature = "custom_allocator")]
        {
            skip = basic_test_fn();
        }

        #[cfg(not(feature = "custom_allocator"))]
        {
            skip = true;
        }
        if !skip {
            // Define a struct
            struct MyStruct {
                field1: i32,
                field2: String,
            }

            // Create an instance of the struct and allocate it on the heap
            let my_struct = MyStruct {
                field1: 10,
                field2: String::from("Hello, Rust!"),
            };

            let boxed_struct = Box::new(my_struct);

            // Use the boxed_struct
            println!("field1: {}", boxed_struct.field1);
            println!("field2: {}", boxed_struct.field2);
        }
    }

    #[test]
    fn test_int() {
        let tag = bindings::tag_enum_TAG_INT;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_int(42 + index as i32, Some(index)).expect("Failed to set int value");
            assert_eq!(vd.get_int(Some(index)), Some(42 + index as i32), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_int8() {
        let tag = bindings::tag_enum_TAG_INT8;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_int8(42 + index as i8, Some(index)).expect("Failed to set int8 value");
            assert_eq!(vd.get_int8(Some(index)), Some(42 + index as i8), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_int16() {
        let tag = bindings::tag_enum_TAG_INT16;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_int16(42 + index as i16, Some(index)).expect("Failed to set int16 value");
            assert_eq!(vd.get_int16(Some(index)), Some(42 + index as i16), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_int32() {
        let tag = bindings::tag_enum_TAG_INT32;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_int32(42 + index as i32, Some(index)).expect("Failed to set int32 value");
            assert_eq!(vd.get_int32(Some(index)), Some(42 + index as i32), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_int64() {
        let tag = bindings::tag_enum_TAG_INT64;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_int64(42 + index as i64, Some(index)).expect("Failed to set int64 value");
            assert_eq!(vd.get_int64(Some(index)), Some(42 + index as i64), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_uint8() {
        let tag = bindings::tag_enum_TAG_UINT8;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_uint8(42 + index as u8, Some(index)).expect("Failed to set uint8 value");
            assert_eq!(vd.get_uint8(Some(index)), Some(42 + index as u8), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_uint16() {
        let tag = bindings::tag_enum_TAG_UINT16;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_uint16(42 + index as u16, Some(index)).expect("Failed to set uint16 value");
            assert_eq!(vd.get_uint16(Some(index)), Some(42 + index as u16), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_uint32() {
        let tag = bindings::tag_enum_TAG_UINT32;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_uint32(42 + index as u32, Some(index)).expect("Failed to set uint32 value");
            assert_eq!(vd.get_uint32(Some(index)), Some(42 + index as u32), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_uint64() {
        let tag = bindings::tag_enum_TAG_UINT64;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_uint64(42 + index as u64, Some(index)).expect("Failed to set uint64 value");
            assert_eq!(vd.get_uint64(Some(index)), Some(42 + index as u64), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_intptr() {
        let tag = bindings::tag_enum_TAG_INTPTR;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_intptr(42 + index as isize, Some(index)).expect("Failed to set intptr value");
            assert_eq!(vd.get_intptr(Some(index)), Some(42 + index as isize), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_uintptr() {
        let tag = bindings::tag_enum_TAG_UINTPTR;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_uintptr(42 + index as usize, Some(index)).expect("Failed to set uintptr value");
            assert_eq!(vd.get_uintptr(Some(index)), Some(42 + index as usize), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_float32() {
        let tag = bindings::tag_enum_TAG_FLOAT32;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_float32(42.0 + index as f32, Some(index)).expect("Failed to set float32 value");
            assert_eq!(vd.get_float32(Some(index)), Some(42.0 + index as f32), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_float64() {
        let tag = bindings::tag_enum_TAG_FLOAT64;
        let num_elements = 5;
        let mut vd = ValueDescriptor::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        for index in 0..num_elements {
            vd.set_float64(42.0 + index as f64, Some(index)).expect("Failed to set float64 value");
            assert_eq!(vd.get_float64(Some(index)), Some(42.0 + index as f64), "Value at index {} does not match expected value", index);
        }

        vd.free();
    }

    #[test]
    fn test_string() {
        let tag = bindings::tag_enum_TAG_STRING;
        let mut vd = ValueDescriptor::new(tag, Some(10))
            .expect("Failed to allocate memory");

        vd.set_string("hello").expect("Failed to set string value");
        assert_eq!(vd.get_string(), Some("hello".to_string()), "String value does not match expected value");

        vd.free();
    }

    #[test]
    fn test_raw_ptr() {
        unsafe {
            // Allocate mem
            let size: usize = 1048;
            let ptr = bindings::sdmm_malloc(size);
            if ptr.is_null() {
                eprintln!("Failed to allocate memory");
                return;
            }
            let value_to_store: i32 = 42;
            let ptr_as_int_ptr = ptr as *mut i32;
            *ptr_as_int_ptr = value_to_store;
            let retrieved_value = *ptr_as_int_ptr;
            assert_eq!(retrieved_value, 42, "Value at adresse {:?} does not match expected value", ptr);
            // let usable_size = bindings::sdmm_malloc_usable_size(ptr);
            let new_size: usize = 2048;
            let new_ptr = bindings::sdmm_realloc(ptr, new_size);
            if new_ptr.is_null() {
                eprintln!("Failed to reallocate memory");
                bindings::sdmm_free(ptr);
                return;
            }    
            let new_ptr_as_int_ptr = new_ptr as *mut i32;
            let preserved_value = *new_ptr_as_int_ptr;
            assert_eq!(preserved_value, 42, "Value at adresse {:?} does not match expected value", new_ptr);    
            let num: usize = 10;
            let element_size: usize = 256;
            let calloc_ptr = bindings::sdmm_calloc(num, element_size);
            if calloc_ptr.is_null() {
                eprintln!("Failed to allocate zero-initialized memory");
                bindings::sdmm_free(new_ptr);
                return;
            }
            println!("Allocated zero-initialized memory for {} elements of size {} at {:?}", num, element_size, calloc_ptr);    
            let calloc_ptr_as_int_ptr = calloc_ptr as *mut i32;
            let zero_value = *calloc_ptr_as_int_ptr;

            assert_eq!(zero_value, 0, "Value at adresse {:?} does not match expected value", calloc_ptr);        
            bindings::sdmm_free(new_ptr);
            bindings::sdmm_free(calloc_ptr);
        }
    }


}


