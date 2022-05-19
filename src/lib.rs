mod solution;

pub use solution::update_inventory;

#[cfg(test)]
mod tests {
    use super::update_inventory;

    #[test]
    fn test_case_1() {
        let cur_inv = vec![];
        let new_inv = vec![];
        let expected_result = vec![];
        let result = update_inventory(cur_inv, new_inv);
        assert_eq!(
            result, expected_result,
            "Empty entries should result in empty outputs"
        );
    }

    #[test]
    fn test_case_2() {
        let cur_inv = vec![
            (21, "Bowling Ball"),
            (2, "Dirty Sock"),
            (1, "Hair Pin"),
            (5, "Microphone"),
        ];
        let new_inv = vec![
            (2, "Hair Pin"),
            (3, "Half-Eaten Apple"),
            (67, "Bowling Ball"),
            (7, "Toothpaste"),
        ];

        let expected_result = vec![
            (88, "Bowling Ball"),
            (2, "Dirty Sock"),
            (3, "Hair Pin"),
            (3, "Half-Eaten Apple"),
            (5, "Microphone"),
            (7, "Toothpaste"),
        ];
        let result = update_inventory(cur_inv, new_inv);
        assert_eq!(
            result, expected_result,
            "The output should reflect the updated values."
        );
    }

    #[test]
    fn test_case_3() {
        let cur_inv = vec![
            (21, "Bowling Ball"),
            (2, "Dirty Sock"),
            (1, "Hair Pin"),
            (5, "Microphone"),
        ];
        let new_inv = vec![];

        let expected_result = vec![
            (21, "Bowling Ball"),
            (2, "Dirty Sock"),
            (1, "Hair Pin"),
            (5, "Microphone"),
        ];
        let result = update_inventory(cur_inv, new_inv);
        assert_eq!(
            result, expected_result,
            "If `new_inv` is empty, the output should be equal to `cur_inv`"
        );
    }

    #[test]
    fn test_case_4() {
        let cur_inv = vec![];
        let new_inv = vec![
            (2, "Hair Pin"),
            (3, "Half-Eaten Apple"),
            (67, "Bowling Ball"),
            (7, "Toothpaste"),
        ];

        let expected_result = vec![
            (67, "Bowling Ball"),
            (2, "Hair Pin"),
            (3, "Half-Eaten Apple"),
            (7, "Toothpaste"),
        ];
        let result = update_inventory(cur_inv, new_inv);
        assert_eq!(
            result, expected_result,
            "If `cur_inv` is empty, the output should be equal to `new_inv`"
        );
    }

    #[test]
    fn test_case_5() {
        let cur_inv = vec![
            (0, "Bowling Ball"),
            (0, "Dirty Sock"),
            (0, "Hair Pin"),
            (0, "Microphone"),
        ];
        let new_inv = vec![
            (1, "Hair Pin"),
            (1, "Half-Eaten Apple"),
            (1, "Bowling Ball"),
            (1, "Toothpaste"),
        ];

        let expected_result = vec![
            (1, "Bowling Ball"),
            (0, "Dirty Sock"),
            (1, "Hair Pin"),
            (1, "Half-Eaten Apple"),
            (0, "Microphone"),
            (1, "Toothpaste"),
        ];
        let result = update_inventory(cur_inv, new_inv);
        assert_eq!(
            result, expected_result,
            "The output should reflect the updated values."
        );
    }
}
