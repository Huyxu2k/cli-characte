pub fn reverse(input:&String)->String{
     return input.chars().rev().collect();
}
pub fn inspects(input :&String,digtis:bool)->((i32,String),String){
    if !digtis{
       return ((input.len() as i32,"".to_string()),String::from("char"));
    }
    return (inspects_num(input),String::from("digit"));
}
pub fn inspects_num(input:&String)->(i32,String){
    let mut count=0;
    let mut rout="".to_string();
    for c in input.chars() {
        if c.is_digit(10){
            count+=1;
            rout=rout +&c.to_string();
        }
    }
    return (count,rout);
}