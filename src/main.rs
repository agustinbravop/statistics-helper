use grouped_data::GroupedData;


pub mod grouped_data;

fn main() {
    println!("Hello, world!");
}

fn group_raw_data(source: &str, intervals: Option<i32>) -> GroupedData<f32> {
    let nums: Vec<f32> = source
        .split_whitespace()
        .map(|el| el.parse::<f32>().unwrap())
        .collect();

    let ni = intervals.unwrap_or_else(
        || intervals_by_sturges(nums.len() as f32).into()
    );
    group_numbers(nums, ni)
}

fn intervals_by_sturges(sample_size: f32) -> u32 {
    (1.0 + 3.3 * sample_size.log10()).round() as u32
}

// fn intervals_by_rice(sample_size: f32) -> u32 {
//     (2.0 * sample_size.cbrt()).round() as u32
// }

fn group_numbers(nums: Vec<f32>, ni: i32) -> GroupedData<f32> {

    

}

