use crate::*;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    let mut index: u8 = 0;
    for test in tests {
        index += 1;
        print!("Executing test {}... ", index);
        test();
        println!("[ok]");
    }
    qemu::exit(qemu::ExitCode::Success);
}

#[test_case]
fn test_print() {
    print!(" ");
}

#[test_case]
fn stress_test_print() {
    const TEST_SIZE: usize = 2000;
    for i in 0..TEST_SIZE {
        println!("print test {}/{}", i, TEST_SIZE);
    }
}
