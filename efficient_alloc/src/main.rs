use std::mem::MaybeUninit;

// run strace:
// strace -o strace.log -s999 -v target/release/effecient_alloc
// search for anonymous allocation:
// grep MAP_ANO strace.log

/* No allocation, seems stack is used
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4c91faa000
mmap(0x7f4c91f7d000, 31600, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x7f4c91f7d000
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f4c91da0000
mmap(NULL, 12288, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_STACK, -1, 0) = 0x7f4c91fc4000
*/
fn maybe_uninit_1mb() {
    let uninit_array: MaybeUninit<[u8; 1024 * 1024]> = MaybeUninit::uninit();
    let mut huge_array = unsafe { uninit_array.assume_init() };

    huge_array[1024 * 512] = 0x55;
    println!("an element of array: {:x}", huge_array[1024 * 512]);
    huge_array[1024 * 1024 - 1] = 0xaa;
    println!("an element of array: {:x}", huge_array[1024 * 1024 - 1]);
}

// strace shows one anonymous allocation of ~ 1 MiB:
// 61:mmap(NULL, 1052672, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7ff50732c000
fn vec_1mb() {
    let mut one_mb_vec: Vec<u8> = Vec::with_capacity(1024 * 1024);
    unsafe { one_mb_vec.set_len(1024 * 1024) }

    one_mb_vec[1024 * 512] = 0x55;
    // one_mb_vec[1024 * 1024 - 1] = 0x55;
    // println!("first element of array: {:x}", one_mb_vec[1024 * 512]);
    println!("an element of array: {:x}", one_mb_vec[1024 * 1024 - 1]);
}

// Should use __rust_alloc_zeroed() which will overcommit memory
// strace shows one anonymous allocation of ~ 1 MiB:
// 61:mmap(NULL, 1052672, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x7f3fbfa8d000
fn vec_macro() {
    let mut one_mb_vec: Vec<u8> = vec![0; 1024 * 1024];

    one_mb_vec[1024 * 512] = 0x55;
    // one_mb_vec[1024 * 1024 - 1] = 0x55;
    // println!("first element of array: {:x}", one_mb_vec[1024 * 512]);
    println!("an element of array: {:x}", one_mb_vec[1024 * 1024 - 1]);
}

fn main() {
    // maybe_uninit_1mb();
    // vec_1mb();
    vec_macro()
}
