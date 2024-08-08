use sdmm_lib::bindings;
use sdmm_lib::test_call_fn;

fn main() {
    #[cfg(feature = "custom_allocator")]
    test_call_fn();

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


    let tag = bindings::tag_enum_TAG_INT;
    let int_container = sdmm_lib::ValueDescriptor::new(tag, None);
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