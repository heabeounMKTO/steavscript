pub enum BscriptTokenType {
    // symbols
    bsKhan, // SEMICOLON (end of line)

    // arithemetics 
    bsBoke, // plus
    bsDork, // minus
    bsKun, // multiply
    bsJaek, //divide 
    
    // misc symbols
    bsKbeas, //comma 
    bsJoch, // literally period
    bsTmenhKadao, // idk how to call this its the ""

    //prims 
    bsLek, //ints
    bsLekKbeas, // floats
    bsPeak, // strings
    bsBool, // ????bools

    //non-prims
    bsBonji, // array
    bsDict, // hashmaps 

    // one or two letter towkens 
    bsMeanDomlai, // set value =
    bsSmer, //equal == 
    bsThomJeang, // bigger than > 
    bsTochJeang, //smaller than <
    bsThomJeangSmer, //bigger than or equal >= 
    bsTochJeangSmer, // smaller than or equal <= 
    bsOrtSmer, // not equal !=

    // keywords
    bsTnak, // CLASS
    bsNeng, // AND 
    bsPsengTeat, // ELSE
    bsKravPiNeng, // OR 
    bsAnukum, // FUNCTIONS
    bsSomrab, // FOR 
    bsTorTe, //NONE 
    bsBer, // IF
    bsMinPit, // FALSE
    bsPit, // TRUE
    bsTrolob, // RETURN
    bsNis, // ThiS or self
    bsAkThe, // VARIABLE
    bsNovPel, // WHILE
    bsJongYeyTha, // PRINT START
    bsJongYeyJengHa, // PRINT END
    bsJob, // EOF 
}


pub struct BscriptToken {
    pub token_type: BscriptTokenType,
    pub lexeme: String,
    pub line: int64
}
