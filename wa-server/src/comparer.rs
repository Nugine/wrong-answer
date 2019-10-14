use crate::types::{Comparer, Comparision, WaResult};

pub struct SimpleComparer {
    pub ignore_trailing_space: bool,
}

impl Comparer for SimpleComparer {
    fn compare(&self, std_answer: &str, user_answer: &str) -> WaResult<Comparision> {
        let std_lines = std_answer.lines();
        let mut user_lines = user_answer.lines();

        for std_line in std_lines {
            let user_line: &str = match user_lines.next() {
                Some(s) => s,
                None => return Ok(Comparision::WA),
            };

            let std_line_trimed = std_line.trim_end();
            let user_line_trimed = user_line.trim_end();
            let std_line_tail = &std_line[std_line_trimed.len()..];
            let user_line_tail = &user_line[user_line_trimed.len()..];

            if std_line_trimed != user_line_trimed {
                return Ok(Comparision::WA);
            }

            if self.ignore_trailing_space {
                continue;
            }

            if std_line_tail != user_line_tail {
                return Ok(Comparision::PE);
            }
        }
        if user_lines.next().is_some() {
            return Ok(Comparision::WA);
        }

        return Ok(Comparision::AC);
    }
}

#[test]
fn test_simple_comparer() {
    let comparer = SimpleComparer {
        ignore_trailing_space: true,
    };

    let ret = comparer.compare("1 2\n3 4", "1 2\r\n3 4\n");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let ret = comparer.compare("1 2 \n3 4", "1 2 \r\n3 4 \n");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let ret = comparer.compare("1 2 \n3 4", "1 2 \r\n3 4 5\n");
    assert_eq!(ret.unwrap(), Comparision::WA);
    let ret = comparer.compare("1 2 \n3 4 5", "1 2 \r\n3 4 \n");
    assert_eq!(ret.unwrap(), Comparision::WA);

    let ret = comparer.compare("\n", "");
    assert_eq!(ret.unwrap(), Comparision::WA);

    let ret = comparer.compare("", "\n");
    assert_eq!(ret.unwrap(), Comparision::WA);

    let ret = comparer.compare(" \n", " ");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let ret = comparer.compare("1\n", "1");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let comparer = SimpleComparer {
        ignore_trailing_space: false,
    };

    let ret = comparer.compare("1 2\n3 4", "1 2\r\n3 4\n");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let ret = comparer.compare("1 2 \n3 4", "1 2 \r\n3 4 \n");
    assert_eq!(ret.unwrap(), Comparision::PE);

    let ret = comparer.compare("1 2 \n3 4", "1 2 \r\n3 4 5\n");
    assert_eq!(ret.unwrap(), Comparision::WA);
    let ret = comparer.compare("1 2 \n3 4 5", "1 2 \r\n3 4 \n");
    assert_eq!(ret.unwrap(), Comparision::WA);

    let ret = comparer.compare("\n", "");
    assert_eq!(ret.unwrap(), Comparision::WA);

    let ret = comparer.compare("", "\n");
    assert_eq!(ret.unwrap(), Comparision::WA);

    let ret = comparer.compare(" \n", " ");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let ret = comparer.compare("1\n", "1");
    assert_eq!(ret.unwrap(), Comparision::AC);

    let ret = comparer.compare("1 \n", "1");
    assert_eq!(ret.unwrap(), Comparision::PE);
}
