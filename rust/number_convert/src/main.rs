#![cfg_attr(not(test), no_main)]



#[no_mangle]
pub extern "C" fn int_number_to_chinese(number: usize) ->  u64 {
    let sum = int_chinese(number);
    let addr = sum.as_ptr() as u64;
    let len = sum.len() as u64;
    let c = len | addr << 8;
    c
}
#[no_mangle]
pub extern "C" fn float_number_to_chinese(number: f64) -> u64 {
    let sum = float_chinese(number);
    let addr = sum.as_ptr() as u64;
    let len = sum.len() as u64;
    let c = len | addr << 8;
    c
}


fn int_chinese(number: usize) -> String {
    let nlen = number.to_string().len();
    let mut sum = String::new();
    if nlen > 8 {
        sum = chn_unit_ch(nlen - 8, &number.to_string());
        sum += "亿";
        sum += &chn_unit_ch(4, &number.to_string()[nlen-8..nlen-4]);
        sum += "万";
        sum += &chn_unit_ch(4, &number.to_string()[nlen-4..]);
    }else  if nlen <= 8 && nlen > 4{
        sum += &chn_unit_ch(nlen-4, &number.to_string());
        sum += "万";
        sum += &chn_unit_ch(4, &number.to_string()[nlen-4..]);
    }else if nlen >=1 && nlen <= 4 {
        sum += &chn_unit_ch(nlen-4, &number.to_string());
    }
    
    sum
}
fn float_chinese(number: f64) -> String {
    let nlen = number.to_string().find('.').unwrap();
    let mut sum = String::new();
    if nlen > 8 {
        sum = chn_unit_ch(nlen - 8, &number.to_string());
        sum += "亿";
        sum += &chn_unit_ch(4, &number.to_string()[nlen-8..nlen-4]);
        sum += "万";
        sum += &chn_unit_ch(4, &number.to_string()[nlen-4..nlen]);
        sum += &chn_unit_ch_float(&number.to_string()[nlen..]);
    }else  if nlen <= 8 && nlen > 4{
        sum += &chn_unit_ch(nlen-4, &number.to_string());
        sum += "万";
        sum += &chn_unit_ch(4, &number.to_string()[nlen-4..nlen]);
        sum += &chn_unit_ch_float(&number.to_string()[nlen..]);
    }else if nlen >=1 && nlen <= 4 {
        sum += &chn_unit_ch(nlen-4, &number.to_string()[..nlen]);
        sum += &chn_unit_ch_float(&number.to_string()[nlen..]);
    }
    sum
}


fn match_number(nchar: char) -> String {
    let ncgar = match nchar{
        '0' => "零".to_string(),
        '1' => "壹".to_string(),
        '2' => "贰".to_string(),
        '3' => "叁".to_owned(),
        '4' => "肆".to_string(),
        '5' => "伍".to_string(),
        '6' => "陆".to_string(),
        '7' => "柒".to_string(),
        '8' => "捌".to_string(),
        '9' => "玖".to_string(),
        '.' => "点".to_string(),
        _ => "".to_string(),
    };
    ncgar
}

fn chn_unit_ch(num: usize, nstr: &str)  ->String{
    if num == 4 {
        return chn_unit_ch_four(&nstr[..4])
    }else if num == 3 {
        return  chn_unit_ch_three(&nstr[..3]);
    }else if num == 2 {
        return chn_unit_ch_two(&nstr[..2]);
    }else if num == 1 {
        return chn_unit_ch_one(&nstr[..1]);
    }
    "error".to_string()
}


fn chn_unit_ch_four(nstr: &str) -> String{
    let mut sum = String::new();
    let arr_string = ["千","百","十"];
    let mut j = 0;
    for i in nstr.chars(){
        if j == 3 && i == '0'{
            break;
        }
         
        sum += &match_number(i);
        if j < 3 && i != '0'{
            sum += arr_string[j];
        }
        j += 1;
    }

    sum
}

fn chn_unit_ch_three(nstr: &str) -> String {
    let mut sum = String::new();
    let arr_string = ["百","十"];
    let mut j = 0;
    for i in nstr.chars(){
        if j == 2 && i == '0'{
            break;
        }
        sum += &match_number(i);
        if j < 2 && i != '0'{
            sum += arr_string[j];
        }
        j += 1;
    }
    sum
}
fn chn_unit_ch_two(nstr: &str) -> String {
    let mut sum = String::new();
    let arr_string = ["十"];
    let mut j = 0;
    for i in nstr.chars(){
        if j == 1 && i == '0'{
            break;
        }
        sum += &match_number(i);
        if j < 1 &&i != '0'{
            sum += arr_string[j];
        }
        j += 1;
    }
    sum
}
fn chn_unit_ch_one(nstr: &str) -> String{
    let mut sum = String::new();
    for i in nstr.chars(){
        sum += &match_number(i);
    }
    sum
}
fn chn_unit_ch_float(nstr: &str) -> String{
    let mut sum = String::new();
    for i in nstr.chars(){
        sum += &match_number(i);
    }
    sum
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integer() {
        assert_eq!("壹千贰百叁十肆万伍千陆百柒十捌", int_chinese(12345678));
        assert_eq!("壹千贰百叁十万伍千陆百柒十", int_chinese(12305670));
        assert_eq!("壹百贰十叁亿零伍百陆十柒万零壹百贰十贰", int_chinese(12305670122));
        assert_eq!("壹百零壹万壹千贰百叁十肆", int_chinese(1011234));
    }
    #[test]
    fn test_float() {
        assert_eq!("壹万贰千壹百贰十壹点贰壹叁", float_chinese(12121.213));
        assert_eq!("壹十贰万零零贰十壹点贰壹", float_chinese(120021.21));
        assert_eq!("壹十贰万壹千零零壹点贰壹", float_chinese(121001.21));
        assert_eq!("壹百贰十叁亿零伍百陆十柒万零壹百贰十贰点贰壹叁", float_chinese(12305670122.213));
    }


}