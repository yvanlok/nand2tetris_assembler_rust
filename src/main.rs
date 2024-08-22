use std::io::Write;
mod parser;
mod code;
mod symbol_table;

enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let in_file_name = args.get(1).expect("No input file provided");

    let mut parser = parser::Parser::new(in_file_name);
    let mut table = symbol_table::SymbolTable::new();

    let out_file_name = in_file_name.replace(".asm", ".hack");

    let mut line_num: usize = 0;
    let mut i = 0;
    while parser.has_more_lines() {
        parser.advance();

        match parser.instruction_type() {
            InstructionType::LInstruction => {
                let symbol = parser.symbol().unwrap();
                if !table.contains(&symbol) {
                    table.add_entry(&symbol, line_num);
                }
            }
            _ => {
                line_num += 1;
            }
        }
        i += 1;
    }

    // Reinitialize parser and open output file
    let mut parser = parser::Parser::new(in_file_name);
    let mut out_file = std::fs::File::create(out_file_name).unwrap();

    while parser.has_more_lines() {
        parser.advance();

        match parser.instruction_type() {
            InstructionType::AInstruction => {
                let symbol = parser.symbol().unwrap();
                if symbol.chars().all(|c| c.is_numeric()) {
                    let symbol = symbol.parse::<usize>().unwrap();
                    let binary = format!("{:016b}", symbol);
                    writeln!(out_file, "{}", binary).unwrap();
                } else {
                    if !table.contains(&symbol) {
                        table.add_entry(&symbol, table.next_space);
                        table.next_space += 1;
                    }
                    let symbol = table.get_address(symbol.as_str()).unwrap();
                    let binary = format!("{:016b}", symbol);
                    writeln!(out_file, "{}", binary).unwrap();
                }
            }
            InstructionType::CInstruction => {
                let comp = parser.comp().unwrap_or_default();
                let mut initial_instruction = String::from("111");

                if comp.contains("M") {
                    initial_instruction.push_str("1");
                } else {
                    initial_instruction.push_str("0");
                }

                let final_instruction = format!(
                    "{}{}{}{}",
                    initial_instruction,
                    code::Code::comp(comp.as_str()),
                    code::Code::dest(parser.dest().as_deref()),
                    code::Code::jump(parser.jump().as_deref())
                );

                writeln!(out_file, "{}", final_instruction).unwrap();
            }
            _ => {}
        }
    }
}
