pub struct Code;

impl Code {
    pub fn dest(mnemonic: Option<&str>) -> &'static str {
        match mnemonic {
            None => "000",
            Some("") => "000",
            Some("M") => "001",
            Some("D") => "010",
            Some("DM") | Some("MD") => "011",
            Some("A") => "100",
            Some("AM") | Some("MA") => "101",
            Some("AD") | Some("DA") => "110",
            Some("ADM") | Some("AMD") | Some("MDA") | Some("MAD") | Some("DMA") | Some("DAM") =>
                "111",
            _ => panic!("Invalid dest mnemonic, {:?}", mnemonic),
        }
    }

    pub fn comp(mnemonic: &str) -> &'static str {
        let mnemonic = mnemonic.trim();
        match mnemonic {
            "0" => "101010",
            "1" => "111111",
            "-1" => "111010",
            "D" => "001100",
            "A" | "M" => "110000",
            "!D" => "001101",
            "!A" | "!M" => "110001",
            "-D" => "001111",
            "-A" | "-M" => "110011",
            "D+1" => "011111",
            "A+1" | "M+1" => "110111",
            "D-1" => "001110",
            "A-1" | "M-1" => "110010",
            "D+A" | "D+M" => "000010",
            "D-A" | "D-M" => "010011",
            "A-D" | "M-D" => "000111",
            "D&A" | "D&M" => "000000",
            "D|A" | "D|M" => "010101",
            _ => panic!("Invalid comp mnemonic: {}", mnemonic),
        }
    }

    pub fn jump(mnemonic: Option<&str>) -> &'static str {
        match mnemonic {
            None => "000",
            Some("") => "000",
            Some("JGT") => "001",
            Some("JEQ") => "010",
            Some("JGE") => "011",
            Some("JLT") => "100",
            Some("JNE") => "101",
            Some("JLE") => "110",
            Some("JMP") => "111",
            _ => panic!("Invalid jump mnemonic"),
        }
    }
}
