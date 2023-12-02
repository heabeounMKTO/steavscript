
pub enum BscriptErrorType {
    BscriptSyntaxError,
    BscriptUnknownError    
}

pub fn BscriptReportError(line: int32  ,error_type: BscriptErrorType ,error_message: String) -> BscriptErrorType{
    println!("{} >> {} : {}", error_type ,line, error_message);
    error_type
}

