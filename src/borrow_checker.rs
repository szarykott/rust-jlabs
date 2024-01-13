pub fn entrypoint() {
    shared_reference();
    thread_and_borrow_checker();
}

fn shared_reference() {
    let mut number = 42;

    let shared_reference_to_number = &number;
    let another_shared_reference_to_number = &number;


    // cannot assign to `*another_shared_reference_to_number`, which is behind a `&` reference
    // `another_shared_reference_to_number` is a `&` reference, so the data it refers to cannot be written
    
    // *another_shared_reference_to_number += 1;


    // cannot borrow `number` as mutable because it is also borrowed as immutable
    // mutable borrow occurs here
    
    // let invalid_to_have_mutable_reference_here = &mut number;

    println!("{shared_reference_to_number}, {another_shared_reference_to_number}");

    let but_valid_here = &mut number;
    *but_valid_here += 1;

    println!("{but_valid_here}")
}

fn thread_and_borrow_checker() {
    use std::thread;

    let val = String::from("value");
    let handle = thread::spawn(move || {
        println!("hello from thread::spawn: {val}");
    });
    handle.join().expect("thread must succeed");

    // borrow of moved value: `val`
    // value borrowed here after move

    // println!("{val}");

    let val = String::from("value");
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from thread::spawn in thread::scope: {val}");
        });
    });

    println!("hello from main thread: {val}");
}