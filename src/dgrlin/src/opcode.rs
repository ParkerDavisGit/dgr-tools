use std::fmt;

use log;

use byteorder::{BigEndian, WriteBytesExt};

#[derive(PartialEq, Debug)]
pub struct Opcode {
    pub name: String,
    pub hexcode: Vec<u8>,
    pub text_id: Option<u16>
}

// Static Functions
impl Opcode {
    // Returns opcode name to be written to file, and the amount of arguments it has.
    pub fn get_opcode_info<'file>(opcode: u8) -> eyre::Result<(&'file str, u8)> {
        let info: (&str, u8) = match opcode {
            0x00 => ("0x00", 2u8),
            0x01 => ("LoadSprite", 3u8),
            0x02 => ("Text", 2u8),
            0x03 => ("TextBoxFormat", 1u8),
            0x04 => ("PostProcessingFilter", 4u8),
            0x05 => ("Movie", 2u8),
            0x06 => ("Animation", 8u8),
            0x08 => ("Voice", 5u8),
            0x09 => ("Music", 3u8),
            0x0a => ("Sound", 3u8),
            0x0b => ("SoundB", 2u8),
            0x0c => ("AddTruthBullets", 2u8),
            0x0d => ("AddPresents", 3u8),
            0x0e => ("UnlockSkill", 2u8),
            0x0f => ("StudentTitleEntry", 3u8),
            0x11 => ("0x11", 4u8),   // "Relationship setting?"  - Morgana-X, Github
            0x14 => ("TrialCamera", 3u8),
            0x15 => ("LoadMap", 3u8),
            0x19 => ("LoadScript", 3u8),
            0x1a => ("StopScript", 0u8),
            0x1b => ("RunScript", 3u8),
            0x1c => ("0x1C", 0u8),
            0x1e => ("Sprite", 5u8),
            0x1f => ("ScreenFlash", 7u8),
            0x20 => ("SpriteFlash", 5u8),
            0x21 => ("Speaker", 1u8),
            0x22 => ("ScreenFade", 3u8),
            0x23 => ("ObjectState", 5u8),
            0x25 => ("ChangeUi", 2u8),
            0x26 => ("SetFlag", 3u8),
            0x27 => ("CheckCharacter", 1u8),
            0x29 => ("CheckObject", 1u8),
            0x2a => ("SetLabel", 2u8),
            0x2b => ("SetChoiceText", 1u8),
            0x2e => ("CameraShake", 2u8),
            0x30 => ("ShowBackground", 3u8),
            0x33 => ("0x33", 4u8),
            0x34 => ("GoToLabel", 2u8),
            0x35 => ("CheckFlagA", 255u8),
            0x36 => ("CheckFlagB", 255u8),
            0x3a => ("WaitInput", 0u8),
            0x3b => ("WaitFrame", 0u8),
            0x3c => ("IfFlagCheck", 0u8),
            _default => {
                eyre::bail!("");
            }
        };

        return Ok(info)
    }
}

impl Opcode {
    pub fn to_hex(&self) -> Vec<u8> {
        self.hexcode.clone()
    }


    pub fn try_from_string(raw_value: String, text_id: u32) -> (Option<Self>, Option<String>) {
        let value = raw_value.trim();

        // Brackets and that newline at the end are to be thrown away
        if value.contains("{") || value.contains("}") || value.len() < 2 {
            return (None, None);
        }

        let mut split_string = value.split("(");
        let opcode_text = split_string.next().unwrap();

        let mut args = split_string.next().unwrap().chars();
        args.next_back(); // Popping off the ending ')'


        let args: Vec<u8> = if opcode_text == "Text" {
            let mut temp: Vec<u8> = Vec::new();
            let _ = temp.write_u16::<BigEndian>(text_id as u16);

            let mut hexcode: Vec<u8> = Vec::new();
            hexcode.push(112u8);
            hexcode.push(2u8);
            hexcode.append(&mut temp);
            
            // Text Line strings must be in double quotes, of course
            if !(args.next_back() == Some('"') && args.next() == Some('"')) {
                return (None, None)
            }

            return (Some(Opcode {
                name: "Text".to_string(),
                hexcode: hexcode,
                text_id: None
            }), Some(args.collect()))
        }
        else {
            args
                .as_str()
                .split(",")
                .flat_map(|line| line.trim().parse::<u8>())
                .collect::<Vec<u8>>()
        };
        
        let opcode: u8 = match opcode_text {
            "0x00"             => 0x00,
            "LoadSprite"       => 0x01,
            "TextBoxFormat"    => 0x03,
            "PostProcessingFilter" => 0x04,
            "Movie"            => 0x05,
            "Animation"        => 0x06,
            "Voice"            => 0x08,
            "Music"            => 0x09,
            "Sound"            => 0x0A,
            "SoundB"           => 0x0B,
            "AddTruthBullets"  => 0x0C,
            "AddPresents"      => 0x0D,
            "UnlockSkill"      => 0x0E,
            "StudentTitleEntry"=> 0x0F,
            "0x11"             => 0x11,
            "TrialCamera"      => 0x14,
            "LoadMap"          => 21u8,
            "LoadScript"       => 0x19,
            "StopScript"       => 0x1A,
            "RunScript"        => 0x1B,
            "0x1C"             => 0x1C,
            "Sprite"           => 30u8,
            "ScreenFlash"      => 31u8,
            "SpriteFlash"      => 32u8,
            "Speaker"          => 0x21,
            "ScreenFade"       => 0x22,
            "ObjectState"      => 0x23,
            "ChangeUi"         => 37u8,
            "SetFlag"          => 38u8,
            "CheckCharacter"   => 39u8,
            "CheckObject"      => 41u8,
            "SetLabel"         => 42u8,
            "SetChoiceText"    => 43u8,
            "CameraShake"      => 0x2E,
            "ShowBackground"   => 48u8,
            "0x33"             => 51u8,
            "GoToLabel"        => 52u8,
            "CheckFlagA"       => 0x35,
            "CheckFlagB"       => 0x36,
            "WaitInput"        => 58u8,
            "WaitFrame"        => 59u8,
            "IfFlagCheck"      => 60u8,
            _badop             => {
                log::error!("INVALID OPCODE - {}", value);
                254u8
            }
        };

        let mut hexcode: Vec<u8> = Vec::new();
        hexcode.push(112u8);
        hexcode.push(opcode);
        hexcode.append(&mut args.clone());

        (Some(Opcode {
            name: opcode_text.to_string(),
            hexcode: hexcode,
            text_id: None
        }), None)
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "{}(", self.name);

        if self.hexcode.len() == 2 {
            let _ = write!(f, ")");
            return Ok(());
        }

        if self.name == "Text" {
            let _ = write!(f, "{})", self.text_id.unwrap());
            return Ok(());
        }

        for idx in 2..(self.hexcode.len()-1) {
            let _ = write!(f, "{}, ", self.hexcode.get(idx).unwrap());
        }

        let _ = write!(f, "{})", self.hexcode.last().unwrap());
        Ok(())
    }
}







//// [ TESTS ] ////
#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;

    use super::Opcode;

    #[test]
    fn test_try_from_string_basic_1() {
        let opcode_1: (Option<Opcode>, Option<String>) 
            = Opcode::try_from_string("WaitFrame()".to_string(), 0);
        
        assert_that!(opcode_1.0.unwrap(), equal_to(
            Opcode {
                name: "WaitFrame".to_string(),
                hexcode: vec![0x70, 0x3B],
                text_id: None,
            }
        ))
    }

    #[test]
    fn test_try_from_string_basic_2() {
        let opcode_1: (Option<Opcode>, Option<String>) 
            = Opcode::try_from_string("Text(\"George\")".to_string(), 260);
        
        assert_that!(opcode_1.0.unwrap(), equal_to(
            Opcode {
                name: "Text".to_string(),
                hexcode: vec![0x70, 0x02, 0x01, 0x04],
                text_id: None,
            }
        ));

        assert_that!(opcode_1.1, equal_to(Some("George".to_string())));
    }

    #[test]
    fn test_try_from_string_basic_3() {
        let opcode_1: (Option<Opcode>, Option<String>) 
            = Opcode::try_from_string("WaitFrame(0, 1, 2, 3, 4)".to_string(), 0);
        

        // Writing these tests helped me realize how flawed this is.
        // I a lot more error checking to tell the user what's wrong with their file.
        // That was supposed to be the point of this project, after all...
        assert_that!(opcode_1.0.unwrap(), equal_to(
            Opcode {
                name: "WaitFrame".to_string(),
                hexcode: vec![0x70, 0x3B, 0x00, 0x01, 0x02, 0x03, 0x04],
                text_id: None,
            }
        ))
    }

    

    #[test]
    #[should_panic]
    fn test_try_from_string_broken_1() {
        #[allow(unused_variables)]
        let opcode_1: (Option<Opcode>, Option<String>) 
            = Opcode::try_from_string("WaitFrame".to_string(), 0);
    }

    #[test]
    //#[should_panic]
    fn test_try_from_string_broken_2() {
        let opcode_1: (Option<Opcode>, Option<String>) 
            = Opcode::try_from_string("Jimmy'sOpcode()".to_string(), 0);
        
        //0xFE is the bad opcode return
        assert_that!(opcode_1.0.unwrap().hexcode[1], equal_to(0xFE));
    }

    
    //#[should_panic]
    // THIS IS A FAIL POINT.
    // IT SHOULD BE MADE TO PANIC
    // But that is something to fix when I change this back into an enum.
    #[test]
    fn test_try_from_string_broken_3() {
        #[allow(unused_variables)]
        let opcode_1: (Option<Opcode>, Option<String>) 
            = Opcode::try_from_string("What the heck si going on(1,| 2,, 3)".to_string(), 0);

            assert_that!(opcode_1.0.unwrap().hexcode, equal_to(vec![0x70, 0xFE, 0x01, 0x03]));
    }

    
}