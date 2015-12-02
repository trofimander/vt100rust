use parser::style::Style;
use parser::style::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum Code {
    Bell, //Bell (Ctrl-G)
    Backspace, //Backspace (Ctrl-H)
    CarriageReturn, //Carriage return (Ctrl-M)
    ReturnTerminalStatus, //Return terminal status (Ctrl-E). Default response is an empty string
    NewLine, //Form Feed (FF), New Page (NP), Line Feed (FF), New Line (NL) or Vertical Tab (Ctrl-K). All are treated the same.
    HorizontalTab, // Horizontal Tab (HT) (Ctrl-I)
    Chars(String), // Plain characters

    // Escape sequences (starting with ESC)
    Index, //IND
    NextLine, //NEL
    HorizontalTabSet, //HTS
    ReverseIndex, //RI
    SingleShiftSelect(u32), //Single Shift Select of G2(G3) Character Set (SS2/SS3). This affects next character only.
    BackIndex,  //Back Index (DECBI), VT420 and up
    SaveCursor, //DECSC
    RestoreCursor,
    ForwardIndex, //Forward Index (DECFI), VT420 and up
    ApplicationKeypad, //Application Keypad (DECKPAM)
    NormalKeypad, //Normal Keypad (DECKPNM)
    CursorToLowerLeft, //Cursor to lower left corner of the screen
    FullReset, //Full Reset (RIS)
    MapCharsetToGL(u32), //Invoke the G2/G3 Character Set as GL - locking shift 2/3 (LS2/LS3)
    MapCharsetToGR(u32), //Invoke the G2/G3 Character Set as GR - locking shift 2/3, right (LS2R/LS3R)

    //Two-char sequences
    Charset7Bit, // 7-bit controls (http://en.wikipedia.org/wiki/ISO/IEC_2022)
    Charset8Bit, //8-bit controls
    AnsiConformanceLevel(u32), //ANSI conformance levels: http://www.vt100.net/docs/vt510-rm/ANSI
    FillScreenE,
    SelectDefaultCharset, //Select default character set. That is ISO 8859-1
    SelectUtf8Charset,
    DesignateCharset(u32, char), //Designate G[0,1,2,3] Character set



    // Control sequences (starting with ESQ[)
    InsertBlankCharacters(u32),
    CursorUp(u32),  //CUU
    CursorDown(u32),  //CUD
    CursorForward(u32), //CUF
    CursorBackward(u32), //CUB
    CursorNextLine(u32),  //CNL
    CursorPrecedingLine(u32),  //CPL
    CursorHorizontalAbsolute(u32), //CHA
    CursorPosition{x:u32, y:u32},  //CUP
    EraseInDisplay(u32), //ED
    SelectiveEraseInDisplay(u32), //DECSED
    EraseInLine(u32), //EL
    SelectiveEraseInLine(u32), //DECSEL
    InsertLines(u32), //IL
    DeleteLines(u32), //DL
    EraseCharacters(u32), //ECH
    DeleteCharacters(u32), //DCH
    ScrollUp(u32), //SU
    ScrollDown(u32), //SD
    SendPrimaryDeviceAttributes, //Primary DA
    SendSecondaryDeviceAttributes, //Secondary DA
    LinePositionAbsolute(u32), //VPA
    TabClear(u32), //TBC
    SetMode(u32), //SM
    SetDecPrivateMode(u32), //DECSet
    ResetMode(u32), //RM
    ResetDECPrivateMode(u32), //DECRST
    CharacterAttributes, //SGR
    RestoreDecPrivateMode, //
    DeviceStatusReport(u32), //DECSTBM
    DecDeviceStatusReport,
    SetScrollingRegion{top:u32, bottom:u32},
    Resize{width:u32, height:u32},

    WindowTitle(String),
    CurrentPath(String),

    //Style
    DefaultStyle,
    StyleOption(Style, bool),
    Foreground(Color),
    Background(Color),


    Error(String)
}
