extern crate hex;

use substring::Substring;
use std::collections::HashMap;



fn main() {
    const QR_CODE : &str = "HC1:NCFOXN%TS3DH3ZSUZK+.V0ETD%65NL-AH-R6IOOK.IR9B+9G4G50PHZF0AT4V22F/8X*G3M9JUPY0BX/KR96R/S09T./0LWTKD33236J3TA3M*4VV2 73-E3GG396B-43O058YIB73A*G3W19UEBY5:PI0EGSP4*2DN43U*0CEBQ/GXQFY73CIBC:G 7376BXBJBAJ UNFMJCRN0H3PQN*E33H3OA70M3FMJIJN523.K5QZ4A+2XEN QT QTHC31M3+E32R44$28A9H0D3ZCL4JMYAZ+S-A5$XKX6T2YC 35H/ITX8GL2-LH/CJTK96L6SR9MU9RFGJA6Q3QR$P2OIC0JVLA8J3ET3:H3A+2+33U SAAUOT3TPTO4UBZIC0JKQTL*QDKBO.AI9BVYTOCFOPS4IJCOT0$89NT2V457U8+9W2KQ-7LF9-DF07U$B97JJ1D7WKP/HLIJL8JF8JFHJP7NVDEBU1J*Z222E.GJ457661CFFTWM-8P2IUE7K*SSW613:9/:TT5IYQBTBU16R4I1A/9VRPJ-TS.7ZEM7MSVOCD4RG2L-TQJROXL2J:52J7F0Q10SMAP3CG3KHF0DWIH";
    let code : &str = QR_CODE.substring(4, QR_CODE.len());
    println!("Taking into account the key : {}", code);

    let decoded_code = match decode_base45(code) {
        Ok(n)=> n,
        Err(_)=>panic!("Couldn't decode the base 45 string")
    };
    println!("Decoded key : {}", decoded_code);

    //let decoded_bytes = hex::decode(decoded_code).expect("Decoding failed");
    //println!("{:?}", decoded_bytes);
}

fn decode_base45(base45_str : &str) -> Result<String, String> {

    // build dictionary with base45 values.
    const BASE45_CHARS : &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:";
    let mut idx :i32 = -1;
    let b45_dico : HashMap<char, i32>= BASE45_CHARS.chars().map(|c| {
        idx +=1;
        return (c,idx)
    }).collect();

    //fill a buffer with input string equivalent values
    let buffer : Vec<i32> = base45_str.chars().map(|c| {
        return b45_dico[&c];
    }).collect();

    let buflen : usize = buffer.len();
    if buflen %3 == 1 {
        return Err("Invalid bae45 string".to_string());
    }

    let mut result_str = String::new();
    for i in (0..buflen).step_by(3) {
        if buflen - i >= 3 {
            let x = buffer.get(i).unwrap_or(&0) + buffer.get(i+1).unwrap_or(&0)*45 + buffer.get(i+2).unwrap_or(&0)*45*45;
            if x > 0xFFFF{
                return Err(" value out of range".to_string());
            }
            result_str.push_str(&format!("{:x}",x));

        } else {
            let x = buffer.get(i).unwrap_or(&0) + buffer.get(i+1).unwrap_or(&0)*45;
            if x > 0xFF{
                return Err(" value out of range".to_string());
            }
            result_str.push_str(&format!("{:x}",x));
        }
    }
    return Ok(result_str);
}