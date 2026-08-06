use super::*;

pub(super) fn generate_steps(app: &mut VisualizerApp) -> Vec<Step> {
    match app.current_problem {
        Problem::HappyNumber => generate_happy_number_steps(19),
        Problem::PlusOne => generate_plus_one_steps(&[1, 2, 3]),
        Problem::SingleNumber => generate_single_number_steps(&[4, 1, 2, 1, 2]),
        Problem::CountingBits => generate_counting_bits_array_steps(5),
        Problem::ReverseBits => generate_reverse_bits_steps(43261596),
        Problem::MissingNumber => generate_missing_number_steps(&[3, 0, 1]),
        Problem::Number1Bits => generate_number_1_bits_steps(11),
        Problem::SumTwoIntegers => generate_sum_two_integers_steps(1, 2),
        Problem::ReverseInteger => generate_reverse_integer_steps(123),
        Problem::RotateImage => {
            generate_rotate_image_steps(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        }
        Problem::SpiralMatrix => {
            generate_spiral_matrix_steps(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        }
        Problem::SetMatrixZeroes => {
            generate_set_matrix_zeroes_steps(&[vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]])
        }
        Problem::PowXN => generate_pow_xn_steps(2.0, 10),
        Problem::MultiplyStrings => generate_multiply_strings_steps("2", "3"),
        Problem::DetectSquares => generate_detect_squares_steps(),
        _ => unreachable!("problem routed to the wrong numeric engine"),
    }
}
