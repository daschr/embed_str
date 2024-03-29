use enc_macros::embed_str;
use std::arch::asm;

#[inline(never)]
unsafe extern "C" fn decrypt(out: *mut String) {
    let mut esp = 0usize;

    // magic offset 0x8c comes from the stack usage of this function, use a dissamber to
    // find the sub esp, [num] at the start of this fn
    asm!("mov {esp}, esp; add {esp}, 0x74", esp = inout(reg) esp);

    esp += 16; // push ebp/ebx/edi/esi

    println!("[decrypt] recovered: {:x}", esp + 0x4);

    let mut eip: *mut u8;
    asm!("mov {eip}, [{esp}]", eip = out(reg) eip, esp = in(reg) esp);

    println!("[decrypt] eip of calling func: {:?}", eip);

    let out = &mut *out;

    let mut eip_c = eip;

    loop {
        let b: u8 = *eip_c;
        if b == 0 {
            eip_c = eip_c.add(1);
            break;
        }
        out.push((b ^ 0xc8) as char);
        eip_c = eip_c.add(1);
    }
    println!(
        "[decrypt] string: \"{}\" eip of next real instruction: {:x?}",
        out, eip_c
    );
    asm!("mov [{esp}], {eip_c}", esp = in(reg) esp, eip_c = in(reg) eip_c);
}

fn main() {
    let mut esp: usize;
    unsafe {
        asm!("mov {esp}, esp" , esp = out(reg) esp);
    }

    println!("real esp: {:x}", esp);
    let s = unsafe { embed_str!("This is some secret string you will never find!") };

    println!("string: {}", s);
}
