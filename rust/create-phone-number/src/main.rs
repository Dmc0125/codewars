fn create_phone_number_v1(numbers: &[u8; 10]) -> String {
    fn convert_arr_to_str(nums: &[u8]) -> String {
        let mut nums_str = String::from("");
        for n in nums {
            let n_ch = char::from_digit(*n as u32, 10).unwrap();
            nums_str.push(n_ch);
        }
        return nums_str
    }

    let preset = convert_arr_to_str(&numbers[0..3]);
    let main_start_part = convert_arr_to_str(&numbers[3..6]);
    let main_end_part = convert_arr_to_str(&numbers[6..10]);

    let phone_number = String::from("(preset) start-end")
        .replace("preset", &preset)
        .replace("start", &main_start_part)
        .replace("end", &main_end_part);

    phone_number
}

fn main() {
    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let phone_number = create_phone_number_v1(&numbers);

    println!("{}", phone_number);
}
