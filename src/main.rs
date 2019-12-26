#[allow(dead_code)]
mod lib_impp;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    mod jni_main_functions {
        use super::*;
        use std::fs;

        #[test]
        fn output_correct() {
            assert!(
                lib_impp::input_to_output("TEST".to_string())
                    == "Input: TEST RUST output".to_string()
            );
        }

        #[test]
        fn return_correct_title() {
            let sample_table =
                String::from(fs::read_to_string("src/tests/sample_table.txt").unwrap());
            assert!(lib_impp::return_title(sample_table) == "IMPP sample table".to_string());
        }

        // Check if result from an import equals our sample json file
        #[test]
        fn import_googlesheet_correct() {
            let sample_table =
                String::from(fs::read_to_string("src/tests/sample_table.txt").unwrap());
            lib_impp::import_googlesheet(sample_table, &"target/");
            assert!(
                String::from(fs::read_to_string("target/database.json").unwrap())
                    == String::from(fs::read_to_string("src/tests/sample_database.json").unwrap())
            );
        }

        #[test]
        fn generate_random_question_number_for_category() {
            assert!(
                lib_impp::generate_random_question(String::from("Endocrinology"), "src/tests/")
                    == 9
            );
        }

        #[test]
        fn get_known_question_details() {
            assert!(
                lib_impp::get_question_details(2, "src/tests/")
                    == ["Fabella sign", "Displacement of the fabella that is seen in cases of synovial effusion and popliteal fossa masses", "Radiologic sign", ""]
            );
        }

        #[test]
        fn count_distractors_all() {
            assert!(
                lib_impp::get_mc_distractors(9, false, "src/tests/")
                    .iter()
                    .filter(|&n| *n == String::from(""))
                    .count()
                    == 4
            );
        }

        #[test]
        fn count_distractors_none() {
            assert!(
                lib_impp::get_mc_distractors(1, false, "src/tests/")
                    .iter()
                    .filter(|&n| *n == String::from(""))
                    .count()
                    == 0
            );
        }

    }
}
