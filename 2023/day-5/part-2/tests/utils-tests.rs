#[path = "../src/utils.rs"]
mod utils;

#[cfg(test)]
mod tests {
  use super::utils::*;

  // overlap

  #[test]
  fn test_overlap_none() {
    let range1 = (10, 20);
    let range2 = (22, 26);

    assert_eq!(overlap(range1, range2), None);
  }

  #[test]
  fn test_overlap_exact() {
    let range = (77, 77);
    let filter = (77, 79);

    assert_eq!(overlap(range, filter), Some((77, 77)));
  }

  #[test]
  fn test_overlap_right() {
    let range1 = (10, 20);
    let range2 = (15, 26);

    assert_eq!(overlap(range1, range2), Some((15, 20)));
  }

  #[test]
  fn test_overlap_left() {
    let range1 = (10, 20);
    let range2 = (5, 15);

    assert_eq!(overlap(range1, range2), Some((10, 15)));
  }

  // subtracts

  #[test]
  fn test_subtracts_none() {
    let range1 = (10, 20);
    let range2 = (5, 25);

    assert_eq!(subtracts(range1, range2), vec![]);
  }

  #[test]
  fn test_subtracts_left() {
    let range1 = (10, 20);
    let range2 = (15, 26);

    assert_eq!(subtracts(range1, range2), vec![(10, 14)]);
  }

  #[test]
  fn test_subtracts_right() {
    let range1 = (10, 20);
    let range2 = (5, 15);

    assert_eq!(subtracts(range1, range2), vec![(16, 20)]);
  }

  #[test]
  fn test_subtracts_both() {
    let range1 = (10, 20);
    let range2 = (12, 16);

    assert_eq!(subtracts(range1, range2), vec![(10, 11), (17, 20)]);
  }

  // range_map

  #[test]
  fn test_range_map_none() {
    let range = (100, 200);
    let map = Map {
      lines: vec![
        Line {
          target_start: 100,
          source_start: 15,
          range: 20,
        },
        Line {
          target_start: 80,
          source_start: 35,
          range: 10,
        },
      ],
    };

    assert_eq!(range_map(range, &map), vec![(100, 200)]);
  }

  #[test]
  fn test_range_map_first() {
    let range = (10, 20);
    let map = Map {
      lines: vec![
        Line {
          target_start: 100,
          source_start: 15,
          range: 20,
        },
        Line {
          target_start: 200,
          source_start: 35,
          range: 82,
        },
      ],
    };

    assert_eq!(range_map(range, &map), vec![(10, 14), (100, 105)]);
  }

  #[test]
  fn test_range_map_second() {
    let range = (35, 50);
    let map = Map {
      lines: vec![
        Line {
          target_start: 100,
          source_start: 15,
          range: 20,
        },
        Line {
          target_start: 200,
          source_start: 35,
          range: 10,
        },
      ],
    };

    assert_eq!(range_map(range, &map), vec![(45, 50), (200, 209)]);
  }

  // ranges_map

  #[test]
  fn test_ranges_map() {
    let ranges = vec![(10, 20), (30, 50)];
    let map = Map {
      lines: vec![
        Line {
          target_start: 100,
          source_start: 15,
          range: 20,
        },
        Line {
          target_start: 200,
          source_start: 35,
          range: 82,
        },
      ],
    };

    assert_eq!(
      ranges_map(ranges, &map),
      vec![(10, 14), (35, 50), (100, 105), (115, 119)]
    );
  }

  // get_lowest

  #[test]
  fn test_get_lowest() {
    let ranges = vec![(30, 35), (40, 50), (10, 20)];

    assert_eq!(get_lowest(ranges), 10);
  }
}
