fn main() {

    let bf_data = include_str!("../hello_world.bf");



    let mut cursor = 0;
    let mut code_cursor = 0;
    let mut memory = vec![0u8; 30_000];

    // Store the 
    let mut block_stack = Vec::new();


    loop{
        if code_cursor == bf_data.len(){
            break;
        }
        let ch = bf_data.chars().nth(code_cursor).unwrap();
        match ch{
            '>' => {    
                cursor += 1;
            },
            '<' => {
                if cursor == 0{
                    cursor = memory.len() - 1;
                }
                else{
                    cursor -= 1;
                }
            },
            '+' => {
                *memory.get_mut(cursor).unwrap() += 1;
            },
            '-' => {
                *memory.get_mut(cursor).unwrap() -= 1;
            },
            '[' => {
                // Check the cursor value, if its > 0, execute until the next ']'
                let cursor_val = memory.get(cursor).unwrap();
                if *cursor_val == 0{
                    // Locate the matching ']'
                    let mut indent = 0u8;
                    let (jump_to_index, _) = bf_data.chars().skip(code_cursor)
                        .enumerate()
                        .find(|(_, c)| match (c, indent){
                            ('[', _) => {
                                indent += 1;
                                false
                            },
                            (']', 1) => {
                                true
                            },
                            (']', _) => {
                                false
                            },
                            (_, _) => false
                        }).unwrap();
                    code_cursor+=jump_to_index+1;
                    continue;
                }
                else{
                    block_stack.push(code_cursor);
                }
            },
            ']' => {
                // Jump to the previous loop open on the stack and remove
                let new_cursor = block_stack.pop().unwrap();
                code_cursor = new_cursor;
                continue;
            },
            '.' => {
                let ch = memory.get(cursor).unwrap();
                use std::io::Write;
                std::io::stdout().write_all(&[*ch]).unwrap();
                std::io::stdout().flush().unwrap();
            },
            ',' => {
                use std::io::Read;
                let mut i = [0u8; 1];
                std::io::stdin().read_exact(&mut i).unwrap();
                *memory.get_mut(cursor).unwrap() = i[0];
            }
            _ => {
                // Comment
            }
        }

        code_cursor += 1;
    }

    println!("\n\n<< PROGRAM COMPLETE >>\n\n")
}
