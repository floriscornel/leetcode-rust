// @leetup=custom
// @leetup=info id=71 lang=rust slug=simplify-path

/*
* Given a string `path`, which is an absolute path (starting with a slash
* `'/'`) to a file or directory in a Unix-style file system, convert it to the
* simplified canonical path.
*
* In a Unix-style file system, a period `'.'` refers to the current directory, a
* double period `'..'` refers to the directory up a level, and any multiple
* consecutive slashes (i.e. `'//'`) are treated as a single slash `'/'`. For this
* problem, any other format of periods such as `'...'` are treated as
* file/directory names.
*
* The canonical path should have the following format:
*
* * The path starts with a single slash `'/'`.
* * Any two directories are separated by a single slash `'/'`.
* * The path does not end with a trailing `'/'`.
* * The path only contains the directories on the path from the root directory to
*   the target file or directory (i.e., no period `'.'` or double period `'..'`)
*
* Return *the simplified canonical path*.
*
*
* Example 1:
*
* Input: path = "/home/"
* Output: "/home"
* Explanation: Note that there is no trailing slash after the last directory n
* ame.
*
* Example 2:
*
* Input: path = "/../"
* Output: "/"
* Explanation: Going one level up from the root directory is a no-op, as the r
* oot level is the highest level you can go.
*
* Example 3:
*
* Input: path = "/home//foo/"
* Output: "/home/foo"
* Explanation: In the canonical path, multiple consecutive slashes are replace
* d by a single one.
*
*
* Constraints:
*
* * `1 <= path.length <= 3000`
* * `path` consists of English letters, digits, period `'.'`, slash `'/'` or
*   `'_'`.
* * `path` is a valid absolute Unix path.
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
    pub fn simplify_path(path: String) -> String {
        let dirs = path.split('/').collect::<Vec<&str>>();
        let mut stack = vec![""];
        for dir in dirs {
            match dir {
                ".." => {
                    stack.pop();
                }
                "." | "" => continue,
                _ => {
                    stack.push(dir);
                }
            }
        }
        let output = "/".to_owned() + &stack.join("/");
        output.replace("//", "/")
    }
}
// @leetup=code

// @leetup=inject:after_code
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::simplify_path("/home/".to_owned()),
            "/home".to_owned()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/".to_owned());
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_owned()),
            "/home/foo".to_owned()
        );
    }
}
// @leetup=inject:after_code
