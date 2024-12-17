fn main() {
    type Instruction = fn(usize, usize, usize, &mut Vec<String>, &mut [usize; 3]) -> usize;

    // Example --
    // let mut registers: [usize; 3] = [729, 0, 0];
    // let program = [0, 1, 5, 4, 3, 0];

    // Output --
    let mut registers: [usize; 3] = [59397658, 0, 0];
    let program = [2, 4, 1, 1, 7, 5, 4, 6, 1, 4, 0, 3, 5, 5, 3, 0];

    // Part one --
    let mut output: Vec<String> = Vec::new();
    let combos = [
        |_| -> usize { 0 },
        |_| -> usize { 1 },
        |_| -> usize { 2 },
        |_| -> usize { 3 },
        |registers: [usize; 3]| -> usize { registers[0] },
        |registers: [usize; 3]| -> usize { registers[1] },
        |registers: [usize; 3]| -> usize { registers[2] },
        |_| -> usize { 7 },
    ];

    #[rustfmt::skip]
    let instructions: [Instruction; 8] = [
        |ptr,    _   , val,   _   , registers| -> usize { registers[0] /= 1 << val                ; ptr + 2 },
        |ptr, operand,  _ ,   _   , registers| -> usize { registers[1] ^= operand                 ; ptr + 2 },
        |ptr,    _   , val,   _   , registers| -> usize { registers[1] = val & 0b111              ; ptr + 2 },
        |ptr, operand,  _ ,   _   , registers| -> usize { if registers[0] == 0 { ptr + 2 } else { operand } },
        |ptr,    _   ,  _ ,   _   , registers| -> usize { registers[1] ^= registers[2]            ; ptr + 2 },
        |ptr,    _   , val, output,     _    | -> usize { output.push((val & 0b111).to_string())  ; ptr + 2 },
        |ptr,    _   , val,   _   , registers| -> usize { registers[1] = registers[0] / (1 << val); ptr + 2 },
        |ptr,    _   , val,   _   , registers| -> usize { registers[2] = registers[0] / (1 << val); ptr + 2 },
    ];

    let mut ptr = 0;
    while ptr < program.len() {
        ptr = instructions[program[ptr]](
            ptr,
            program[ptr + 1],
            combos[program[ptr + 1]](registers),
            &mut output,
            &mut registers,
        );
    }

    // println!("registers: {:?}", registers);
    println!("Part one: {}", output.join(","));
}
