use crate::sdmm_mod::value::TaggedValue;
use crate::sdmm_mod::bindings::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let tag = bindings::tag_enum_TAG_INT;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_int(42 + i as i32, Some(i)).expect("Failed to set int value");
            assert_eq!(tagged_value.get_int(Some(i)), Some(42 + i as i32), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_int8() {
        let tag = bindings::tag_enum_TAG_INT8;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_int8(42 + i as i8, Some(i)).expect("Failed to set int8 value");
            assert_eq!(tagged_value.get_int8(Some(i)), Some(42 + i as i8), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_int16() {
        let tag = bindings::tag_enum_TAG_INT16;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_int16(42 + i as i16, Some(i)).expect("Failed to set int16 value");
            assert_eq!(tagged_value.get_int16(Some(i)), Some(42 + i as i16), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_int32() {
        let tag = bindings::tag_enum_TAG_INT32;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_int32(42 + i as i32, Some(i)).expect("Failed to set int32 value");
            assert_eq!(tagged_value.get_int32(Some(i)), Some(42 + i as i32), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_int64() {
        let tag = bindings::tag_enum_TAG_INT64;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_int64(42 + i as i64, Some(i)).expect("Failed to set int64 value");
            assert_eq!(tagged_value.get_int64(Some(i)), Some(42 + i as i64), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_uint8() {
        let tag = bindings::tag_enum_TAG_UINT8;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_uint8(42 + i as u8, Some(i)).expect("Failed to set uint8 value");
            assert_eq!(tagged_value.get_uint8(Some(i)), Some(42 + i as u8), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_uint16() {
        let tag = bindings::tag_enum_TAG_UINT16;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_uint16(42 + i as u16, Some(i)).expect("Failed to set uint16 value");
            assert_eq!(tagged_value.get_uint16(Some(i)), Some(42 + i as u16), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_uint32() {
        let tag = bindings::tag_enum_TAG_UINT32;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_uint32(42 + i as u32, Some(i)).expect("Failed to set uint32 value");
            assert_eq!(tagged_value.get_uint32(Some(i)), Some(42 + i as u32), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_uint64() {
        let tag = bindings::tag_enum_TAG_UINT64;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_uint64(42 + i as u64, Some(i)).expect("Failed to set uint64 value");
            assert_eq!(tagged_value.get_uint64(Some(i)), Some(42 + i as u64), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_intptr() {
        let tag = bindings::tag_enum_TAG_INTPTR;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_intptr(42 + i as isize, Some(i)).expect("Failed to set intptr value");
            assert_eq!(tagged_value.get_intptr(Some(i)), Some(42 + i as isize), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_uintptr() {
        let tag = bindings::tag_enum_TAG_UINTPTR;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_uintptr(42 + i as usize, Some(i)).expect("Failed to set uintptr value");
            assert_eq!(tagged_value.get_uintptr(Some(i)), Some(42 + i as usize), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_float32() {
        let tag = bindings::tag_enum_TAG_FLOAT32;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_float32(42.0 + i as f32, Some(i)).expect("Failed to set float32 value");
            assert_eq!(tagged_value.get_float32(Some(i)), Some(42.0 + i as f32), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_float64() {
        let tag = bindings::tag_enum_TAG_FLOAT64;
        let num_elements = 5;
        let mut tagged_value = TaggedValue::new(tag, Some(num_elements))
            .expect("Failed to allocate memory");

        // Set and get values
        for i in 0..num_elements {
            tagged_value.set_float64(42.0 + i as f64, Some(i)).expect("Failed to set float64 value");
            assert_eq!(tagged_value.get_float64(Some(i)), Some(42.0 + i as f64), "Value at index {} does not match expected value", i);
        }

        // Free memory
        tagged_value.free();
    }

    #[test]
    fn test_string() {
        let tag = bindings::tag_enum_TAG_STRING;
        let mut tagged_value = TaggedValue::new(tag, Some(10))
            .expect("Failed to allocate memory");

        tagged_value.set_string("hello").expect("Failed to set string value");
        assert_eq!(tagged_value.get_string(), Some("hello".to_string()), "String value does not match expected value");

        // Free memory
        tagged_value.free();
    }
}
