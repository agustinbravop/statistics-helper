pub struct GroupedData<T> {
  min: T,
  max: T,
  intervals: Vec<ClassInterval<T>>,
  pos: Positions,
  disps: Dispersions
}

struct ClassInterval<T> {
  width: f32,
  lower_limit: T,
  upper_limit: T,
  actual_lower_limit: f32,
  skewness: f32,
  class_mark: f32,
  freqs: Frequencies,
}

struct Frequencies {
  abs: u32,
  abs_acum: u32,
  rel: f32,
  rel_acum: f32
}

struct Positions {
  median: f32,
  mode: f32,
  mean: f32,
  first_quartile: f32,
  third_quartile: f32
}

struct Dispersions {
  range: f32,
  variance: f32,
  standard_deviation: f32,
  coefficient_of_variation: f32,
}
