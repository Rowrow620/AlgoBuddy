mod code_lines;
mod details;

pub use code_lines::get_code_lines;
pub use details::get_details;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::problem::{Problem, ProblemDetails};

    const GRAPH_PROBLEMS: [Problem; 13] = [
        Problem::NumberIslands,
        Problem::MaxAreaIsland,
        Problem::CloneGraph,
        Problem::WallsAndGates,
        Problem::RottingOranges,
        Problem::PacificAtlantic,
        Problem::SurroundedRegions,
        Problem::CourseSchedule,
        Problem::CourseScheduleII,
        Problem::GraphValidTree,
        Problem::ConnectedComponents,
        Problem::RedundantConnection,
        Problem::WordLadder,
    ];

    fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= u64::from(*byte);
            *hash = hash.wrapping_mul(1_099_511_628_211);
        }
    }

    fn hash_usize(hash: &mut u64, value: usize) {
        hash_bytes(hash, &value.to_le_bytes());
    }

    fn hash_str(hash: &mut u64, value: &str) {
        hash_usize(hash, value.len());
        hash_bytes(hash, value.as_bytes());
    }

    fn details_fingerprint(details: &ProblemDetails) -> u64 {
        let mut hash = 14_695_981_039_346_656_037;
        hash_bytes(&mut hash, &details.id.to_le_bytes());
        hash_str(&mut hash, details.title);
        hash_str(&mut hash, &format!("{:?}", details.difficulty));
        hash_str(&mut hash, &format!("{:?}", details.category));
        hash_str(&mut hash, details.statement);
        for example in details.examples {
            hash_str(&mut hash, example.input);
            hash_str(&mut hash, example.output);
            hash_str(&mut hash, example.explanation);
        }
        for constraint in details.constraints {
            hash_str(&mut hash, constraint);
        }
        hash_str(&mut hash, details.leetcode_url);
        for approach in details.approaches {
            hash_usize(&mut hash, approach.id);
            hash_str(&mut hash, approach.name);
            hash_str(&mut hash, approach.time_complexity);
            hash_str(&mut hash, approach.space_complexity);
            hash_str(&mut hash, approach.rationale);
            hash_str(&mut hash, approach.description);
        }
        hash
    }

    fn code_fingerprint(lines: &[(usize, &'static str)]) -> u64 {
        let mut hash = 14_695_981_039_346_656_037;
        for (line_number, text) in lines {
            hash_usize(&mut hash, *line_number);
            hash_str(&mut hash, text);
        }
        hash
    }

    const EXPECTED: [(u64, usize, usize, u64, usize); 13] = [
        (
            13_143_984_877_894_057_597,
            1,
            3,
            15_576_473_959_952_876_237,
            16,
        ),
        (
            8_988_876_803_082_220_627,
            1,
            2,
            8_529_609_668_582_195_542,
            11,
        ),
        (
            16_937_690_465_296_879_931,
            1,
            1,
            15_972_232_699_843_609_587,
            10,
        ),
        (
            7_317_828_574_140_707_477,
            1,
            2,
            12_612_393_636_042_823_444,
            16,
        ),
        (
            7_897_707_488_146_041_909,
            1,
            2,
            2_163_715_153_980_503_814,
            17,
        ),
        (
            16_378_626_496_600_770_538,
            1,
            2,
            14_513_051_853_891_495_914,
            10,
        ),
        (
            17_075_019_069_189_364_948,
            1,
            2,
            4_662_996_018_060_675_329,
            13,
        ),
        (
            7_766_221_265_427_045_822,
            1,
            1,
            16_017_510_259_662_730_099,
            16,
        ),
        (
            12_433_807_468_900_744_238,
            1,
            1,
            8_129_124_795_404_257_070,
            16,
        ),
        (
            9_872_938_091_780_849_814,
            1,
            1,
            5_146_247_164_223_698_453,
            14,
        ),
        (
            8_053_929_240_578_018_320,
            1,
            1,
            13_974_767_916_828_500_742,
            16,
        ),
        (
            4_349_749_739_450_088_713,
            1,
            2,
            3_104_113_726_819_350_255,
            15,
        ),
        (
            6_290_970_733_707_778_450,
            1,
            2,
            17_429_507_471_773_642_643,
            19,
        ),
    ];

    #[test]
    fn graph_metadata_keeps_its_exact_content() {
        for (problem, expected) in GRAPH_PROBLEMS.into_iter().zip(EXPECTED) {
            let details = get_details(problem).expect("graph problem must have details");
            let code = get_code_lines(problem, 0).expect("graph problem must have code lines");
            assert_eq!(get_code_lines(problem, usize::MAX), Some(code.clone()));
            assert_eq!(
                (
                    details_fingerprint(&details),
                    details.examples.len(),
                    details.constraints.len(),
                    code_fingerprint(&code),
                    code.len()
                ),
                expected,
                "metadata changed for {problem:?}"
            );
        }
        assert!(get_details(Problem::ContainsDuplicate).is_none());
        assert!(get_code_lines(Problem::ContainsDuplicate, 0).is_none());
    }
}
