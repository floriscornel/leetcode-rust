// @leetup=custom
// @leetup=info id=93 lang=rust slug=restore-ip-addresses

/*
* A valid IP address consists of exactly four integers separated by single
* dots. Each integer is between `0` and `255` (inclusive) and cannot have
* leading zeros.
*
* * For example, `"0.1.2.201"` and `"192.168.1.1"` are valid IP addresses, but
*   `"0.011.255.245"`, `"192.168.1.312"` and `"192.168@1.1"` are invalid IP
*   addresses.
*
* Given a string `s` containing only digits, return *all possible valid IP
* addresses that can be formed by inserting dots into *`s`. You are not
* allowed to reorder or remove any digits in `s`. You may return the valid IP
* addresses in any order.
*
*
* Example 1:
*
* Input: s = "25525511135"
* Output: ["255.255.11.135","255.255.111.35"]
*
* Example 2:
*
* Input: s = "0000"
* Output: ["0.0.0.0"]
*
* Example 3:
*
* Input: s = "101023"
* Output: ["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"]
*
*
* Constraints:
*
* * `1 <= s.length <= 20`
* * `s` consists of digits only.
*
*/
// @leetup=custom
// @leetup=inject:before_code_ex
// before_code_exclude
#![allow(dead_code, unused_variables)]
struct Solution {}
// @leetup=inject:before_code_ex

// @leetup=code
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result = Vec::new();
        if s.len() < 4 || s.len() > 12 {
            return result;
        }
        for i in 1..s.len() - 2 {
            for j in (i + 1)..s.len() - 1 {
                for k in (j + 1)..s.len() {
                    let ip = [&s[..i], &s[i..j], &s[j..k], &s[k..]].map(|s| s.parse::<u8>());
                    if ip.iter().all(|n| n.is_ok()) {
                        let ip_string = ip
                            .iter()
                            .map(|n| n.as_ref().unwrap().to_string())
                            .collect::<Vec<String>>()
                            .join(".");
                        if ip_string.len() == s.len() + 3 {
                            result.push(ip_string)
                        }
                    }
                }
            }
        }
        result
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let s = "25525511135";
        let expected = ["255.255.11.135", "255.255.111.35"];
        assert_eq!(
            Solution::restore_ip_addresses(s.to_string()),
            expected
                .to_vec()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn example_2() {
        let s = "0000";
        let expected = ["0.0.0.0"];
        assert_eq!(
            Solution::restore_ip_addresses(s.to_string()),
            expected
                .to_vec()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn example_3() {
        let s = "101023";
        let expected = [
            "1.0.10.23",
            "1.0.102.3",
            "10.1.0.23",
            "10.10.2.3",
            "101.0.2.3",
        ];
        assert_eq!(
            Solution::restore_ip_addresses(s.to_string()),
            expected
                .to_vec()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn example_4() {
        let s = "11111111";
        let expected = [
            "1.1.111.111",
            "1.11.11.111",
            "1.11.111.11",
            "1.111.1.111",
            "1.111.11.11",
            "1.111.111.1",
            "11.1.11.111",
            "11.1.111.11",
            "11.11.1.111",
            "11.11.11.11",
            "11.11.111.1",
            "11.111.1.11",
            "11.111.11.1",
            "111.1.1.111",
            "111.1.11.11",
            "111.1.111.1",
            "111.11.1.11",
            "111.11.11.1",
            "111.111.1.1",
        ];
        assert_eq!(
            Solution::restore_ip_addresses(s.to_string()),
            expected
                .to_vec()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn example_5() {
        let s = "0";
        assert_eq!(
            Solution::restore_ip_addresses(s.to_string()),
            Vec::<String>::new()
        );
    }
}
// @leetup=inject:after_code
