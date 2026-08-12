use crate::app::VisualizerApp;
use crate::model::Problem;

pub(super) fn i32_list(
    app: &VisualizerApp,
    problem: Problem,
    key: &'static str,
    fallback_text: &'static str,
    fallback_values: &[i32],
) -> Vec<i32> {
    crate::utils::parse_i32_vec(
        app.get_input_str(problem, key, fallback_text),
        fallback_values,
    )
}

pub(super) fn string_list(
    app: &VisualizerApp,
    problem: Problem,
    key: &'static str,
    fallback_text: &'static str,
    fallback_values: &[&str],
) -> Vec<String> {
    crate::utils::parse_string_vec(
        app.get_input_str(problem, key, fallback_text),
        fallback_values,
    )
}

pub(super) fn string_list_allow_empty(
    app: &VisualizerApp,
    problem: Problem,
    key: &'static str,
    fallback_text: &'static str,
) -> Vec<String> {
    app.get_input_str(problem, key, fallback_text)
        .split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_lists_use_fallback_only_when_no_number_parses() {
        let mut app = VisualizerApp::default();
        app.set_input_str(Problem::TwoSum, "nums", "4, invalid, 7");
        assert_eq!(
            i32_list(&app, Problem::TwoSum, "nums", "1, 2", &[1, 2]),
            vec![4, 7]
        );

        app.set_input_str(Problem::TwoSum, "nums", "invalid");
        assert_eq!(
            i32_list(&app, Problem::TwoSum, "nums", "1, 2", &[1, 2]),
            vec![1, 2]
        );
    }

    #[test]
    fn string_lists_trim_values_and_preserve_fallbacks() {
        let mut app = VisualizerApp::default();
        app.set_input_str(Problem::EncodeDecode, "strs", "alpha, , beta");
        assert_eq!(
            string_list(
                &app,
                Problem::EncodeDecode,
                "strs",
                "fallback",
                &["fallback"]
            ),
            vec!["alpha", "beta"]
        );

        app.set_input_str(Problem::EncodeDecode, "strs", " , ");
        assert_eq!(
            string_list(
                &app,
                Problem::EncodeDecode,
                "strs",
                "fallback",
                &["fallback"]
            ),
            vec!["fallback"]
        );

        assert!(
            string_list_allow_empty(&app, Problem::EncodeDecode, "strs", "fallback").is_empty()
        );
    }
}
