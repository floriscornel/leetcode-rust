# LeetCode challenges in Rust

These are solutions in Rust for LeetCode challenges. 

## Setup
Make sure to have rust installed and updated to at least version `1.5.60`. To install and update rust check out [rustup](https://rustup.rs/).

I am using [Visual Studio Code](https://code.visualstudio.com/) with the [rust-analyer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer&ssr=true) extension.

## CLI tool leetup 
LeetCode challenges can be downloaded, tested and submitted using the cli tool [leetup](https://github.com/dragfire/leetup).

This is the configuration file that I (`~/.leetup/config.json`), the `work_dir` should be adjusted when copying it.
```json
{
    "pick_hook": {
        "rust": {
            "working_dir": "/home/user/Projects/leetcode-rust/src/problems",
            "script": {
                "pre_generation": [
                    "echo '#[path = \"@leetup=problem.rs\"]' >> @leetup=working_dir/mod.rs",
                    "echo 'mod @leetup=problem;' | tr '-' '_' >> @leetup=working_dir/mod.rs"
                ],
                "post_generation": [
                    "cd @leetup=working_dir && cargo fmt"
                ]
            }
        }
    },
    "inject_code": {
        "rust": {
            "before_code_exclude": [
                "// before_code_exclude",
                "#![allow(dead_code, unused_variables)]",
                "struct Solution {}"
            ],
            "after_code": [
                "#[cfg(test)]",
                "mod tests {",
                "    use super::Solution;",
                "    #[test]",
                "    fn example_1() {",
                "        ",
                "    }",
                "}"
            ]
        }
    }
}
```

After installing `leetup` authenticate your leetcode account and create the configuration file.

The following commands can be used to interact with LeetCode from the CLI:

1. Downloading a challenge by the LeetCode ID: 
   
   $ `leetup pick 136`

2. Testing a file remotely for a specific input: 
   
   $ `leetup leetup test src/problems/single-number.rs -t "[2,2,1]"`

   Will result in:
    ```
    ✔ Accepted
    Input:
    [2,2,1]

    Output:
    1

    Expected:
    1
    ```

3. Submitting a problem

    $ `leetup submit src/problems/single-number.rs`

    ```
    ✔ Accepted
    61/61 cases passed (7 ms)
    Your runtime beats 23.2805% of rust submissions
    Your memory usage beats 25.926% of rust submissions (2.3 MB)
    ```