mod safe_sdmm;
use crate::safe_sdmm::value::TaggedValue;
use crate::safe_sdmm::bindings::*;


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
    let tag = bindings::tag_enum_TAG_INT;
    let int_container = TaggedValue::new(tag, None);
    if int_container.is_none() {
        eprintln!("Failed to allocate memory");
        return;
    }
    let mut int_container = int_container.unwrap();

    println!("Allocated memory with tag {:?} at {:?}", int_container.tag, unsafe { int_container.value._int_ });

    int_container.set_int(42, None).expect("Failed to set int value");

    if let Some(value) = int_container.get_int(None) {
        println!("Stored int value: {}", value);
    } else {
        eprintln!("Failed to retrieve int value");
    }
}