pub enum Op {
    Zero {
        args: Vec<u8>,
    },
    ScreenFade {
        args: Vec<u8>,
    },
    ChangeUi {
        args: Vec<u8>,
    },
    ThreeThree {
        args: Vec<u8>,
    },
    SetFlag {
        args: Vec<u8>,
    },
    LoadMap {
        args: Vec<u8>,  // MAP ID, 
    },
    Sprite {
        args: Vec<u8>, // ???, CHAR ID, SPRITE ID, ?Visible?, ???
    },
    Music {
        args: Vec<u8>,
    },
    ShowBackground {
        args: Vec<u8>,
    },
    Speaker {
        arg: u8, // Speaker ID
    },
    TextBoxFormat { // 03
        arg: u8, // Speaker ID
    },
    Text { 
        args: Vec<u8>,
    },
    WaitFrame,
    WaitInput,
    Sound { 
        args: Vec<u8>,
    }, 
    ScreenFlash {
        args: Vec<u8>, // 7
    },
    Animation {
        args: Vec<u8>, // 8
    },
    Voice { 
        args: Vec<u8>, // 5
    },
    SpriteFlash { 
        args: Vec<u8>, // 5
    },
    GoToLabel {
        args: Vec<u8>, // 2
    },
    CheckCharacter {
        args: Vec<u8>, // 1
    },
    CheckObject {
        aargs: Vec<u8>, // 1
    },
    CheckFlagA {
        args: Vec<u8>, // Varies
    },
    If_FlagCheck,
    SetLabel { 
        args: Vec<u8>, // 2
    },
    SetChoiceText {
        args: Vec<u8>, // 1
    },
    LoadScript {
        args: Vec<u8>, // 3
    },
    StopScript,
}