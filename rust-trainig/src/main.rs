fn main() {
    let mut _celc_temp: i32;
    let celc_range: [i32; 5] = [-100, -50, 0, 50, 100];
    let fahre_range:[i32;6] = [0,20,40,60,80,100];

    for temp in celc_range {
        println!("{}°cels = {}°fahre", temp, celc_convertor_to_fahre(temp));
    }
    for temp in fahre_range {
        println!("{}°fahre = {}°celc", temp, fahre_convertor_to_celc(temp));
    }


}

fn celc_convertor_to_fahre(temp: i32) -> i32 {
    let result: i32 = ((temp * 9) / 5) + 32;

    return result;
}

fn fahre_convertor_to_celc(temp: i32) -> i32 {
    let result: i32 = ((temp -32) * 5) / 9;

    return result;
}
