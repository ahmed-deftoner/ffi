extern crate libc;

extern "C" {
    fn triple_input(input: libc::c_int) -> libc::c_int;
    fn quickSort(arr: *const libc::c_int, start: libc::c_int, end: libc::c_int);
}

fn main() {
    let input = 4;
    let output = unsafe { triple_input(input) };
    unsafe {
        let mut arr = [ 9, 3, 4, 2, 1, 8 ].to_vec();
        let mut n = 6;
        quickSort(arr.as_mut_ptr(), 0, n - 1);
        println!("{:?}", arr);
    };
    println!("{} * 3 = {}", input, output);
}