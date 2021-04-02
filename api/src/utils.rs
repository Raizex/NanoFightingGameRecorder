use rand::Rng;
use std::vec::Vec;

//Function generates a unique key
pub fn generate() -> String{

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const PASSWORD_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    return password;
}

// Function converts time (in ms) to a Vector with minutes and seconds
pub fn convert_time(time: i64) -> Vec<i64>{

    let mut minutes = 0;
    let mut seconds = 0;
    let mut time_list: Vec<i64> = Vec::new();

    //Convert time to seconds (time should be in miliseconds)
    let time = time / 1000;

    //Check if time is less than a minute
    //If so, just update the seconds variable
    //If not, update minutes and seconds
    if time <= 60 {
       seconds = time;
    }else{
        minutes = time / 60;
        seconds = time % 60;
    }

    //Append minutes and seconds to the list
    time_list.push(minutes);
    time_list.push(seconds);

    return time_list
}
