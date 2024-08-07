use sdmm_lib::bindings;

fn main() {
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