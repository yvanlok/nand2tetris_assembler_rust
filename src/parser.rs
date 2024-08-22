use std::io::{ BufRead, BufReader, Seek, SeekFrom };
use std::fs::File;
use crate::InstructionType;

#[derive(Debug)]
pub struct Parser {
    pub file: File,
    pub reader: BufReader<File>,
    pub current_instruction: Option<String>,
}

impl Parser {
    pub fn new(file_name: &str) -> Parser {
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file.try_clone().unwrap());
        Parser {
            file,
            reader,
            current_instruction: None,
        }
    }

    pub fn has_more_lines(&mut self) -> bool {
        let current_position = self.reader.seek(SeekFrom::Current(0)).unwrap(); // Save current reader position
        let mut buffer = String::new();
        let bytes_read = self.reader.read_line(&mut buffer).unwrap(); // Try to read the next line
        self.reader.seek(SeekFrom::Start(current_position)).unwrap(); // Restore the original position
        bytes_read > 0 // Return true if bytes were read, indicating more lines
    }

    pub fn advance(&mut self) {
        let mut buffer = String::new();
        while self.reader.read_line(&mut buffer).unwrap() > 0 {
            let curr_line = buffer.trim().to_string();
            buffer.clear(); // Clear buffer for the next read

            if curr_line.starts_with("//") {
                continue; // Skip comment lines
            } else if !curr_line.is_empty() {
                self.current_instruction = Some(curr_line);
                return;
            }
        }
        self.current_instruction = None; // No more instructions to process
    }

    pub fn instruction_type(&self) -> InstructionType {
        if let Some(curr_instruction) = &self.current_instruction {
            let curr_instruction = curr_instruction.trim();
            if curr_instruction.starts_with('@') {
                InstructionType::AInstruction
            } else if curr_instruction.starts_with('(') && curr_instruction.ends_with(')') {
                InstructionType::LInstruction
            } else {
                InstructionType::CInstruction
            }
        } else {
            panic!("No current instruction");
        }
    }

    pub fn symbol(&self) -> Option<String> {
        if let Some(curr_instruction) = &self.current_instruction {
            let len = curr_instruction.len();
            match self.instruction_type() {
                InstructionType::AInstruction => Some(curr_instruction[1..].to_string()),
                InstructionType::LInstruction => Some(curr_instruction[1..len - 1].to_string()),
                _ => None,
            }
        } else {
            panic!("No current instruction");
        }
    }

    pub fn dest(&self) -> Option<String> {
        if let Some(curr_instruction) = &self.current_instruction {
            if curr_instruction.contains('=') {
                Some(curr_instruction.split('=').collect::<Vec<&str>>()[0].to_string())
            } else {
                None
            }
        } else {
            panic!("No current instruction");
        }
    }

    pub fn comp(&self) -> Option<String> {
        if let Some(curr_instruction) = &self.current_instruction {
            let curr_instruction = curr_instruction.trim();
            if curr_instruction.is_empty() {
                return None;
            }

            let dest_split: Vec<&str> = curr_instruction.split('=').collect();
            let comp_part = if dest_split.len() > 1 {
                dest_split[1].trim()
            } else {
                dest_split[0].trim()
            };

            let jump_split: Vec<&str> = comp_part.split(';').collect();
            Some(jump_split[0].trim().to_string())
        } else {
            None
        }
    }

    pub fn jump(&self) -> Option<String> {
        if let Some(curr_instruction) = &self.current_instruction {
            if curr_instruction.contains(';') {
                Some(curr_instruction.split(';').collect::<Vec<&str>>()[1].to_string())
            } else {
                None
            }
        } else {
            panic!("No current instruction");
        }
    }
}
