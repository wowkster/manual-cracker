use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::Write,
};

// 2022: W3lc0m3B^Ck2022!
// 2020: R!$E2geTHeR#2020
// 2019: $Robots&in#SPACE!!
// 2018: pLaY&4%R3aL!
// 2017: &Full$team^Ahead!
// 2016: @Ahead)Together!FRC^2016
// 2015: R3C3CL3RU$H2015
// 2014: 3Zones2Goals1Alliance!

// - All English Words
// - Mixed Case
// - More often than not, 16 characters
// - Usually ends in a !
// - Usually has the year at or near the end
// - Uses common character replacements
// - Words are sometimes delimited by symbols

const PASSWORD_LENGTH: u32 = 16;
const CURRENT_YEAR: &str = "2023";

lazy_static::lazy_static! {
    static ref CHAR_REPLACEMENTS: HashMap<char, Vec<char>> = HashMap::from([
        ('a', vec!['4', '^']),
        ('e', vec!['3']),
        ('i', vec!['!']),
        ('o', vec!['0']),
        ('s', vec!['$']),
        ('y', vec!['3']),
    ]);
}

fn main2() {
    let dict = std::fs::read_to_string("./wordlist.10000.txt")
        .expect("Could not read `./wordlist.10000.txt`");
    let wordlist = dict.lines();

    let mut output =
        std::fs::File::create("wordlist.txt").expect("Cannot open file `wordlist.txt` for writing");

    const LINES: u32 = 10_000;

    let mut words_processed = 0;

    for word in wordlist {
        words_processed += 1;

        if words_processed % 50 == 0 {
            println!(
                "Words Processed: {} | Percent Completed: {:.2?}%",
                words_processed,
                (words_processed as f32 / LINES as f32) * 100.0
            )
        }

        if word.len() > 8 {
            continue;
        }

        output.write(word.as_bytes()).unwrap();
        output.write(b"\n").unwrap();
    }
}

fn main() {
    let dict = std::fs::read_to_string("./wordlist.txt").expect("Could not read `./wordlist.txt`");
    let wordlist: Vec<_> = dict.lines().collect();

    let wordlist = vec![
        "charged", "up", // "ahead", "rise", "together", "2gether", "frc", "robots",
    ];

    let delimiters = [
        '#', '$', '&', '#', '!', '^', '@', '(', ')', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        '0',
    ];

    let mut output =
        std::fs::File::create("output.txt").expect("Cannot open file `output.dict` for writing");

    println!("Creating word permutations:");
    let permutations = generate_word_permutations(&wordlist);

    println!("Creating sentence combinations:");
    let sentences = create_word_pairs(&wordlist);

    println!("Sentences: {sentences:?}");

    let mut passwords: VecDeque<String> = VecDeque::new();

    for sentence in sentences {
        let word1_perms = permutations.get(*sentence[0]).unwrap();
        let word2_perms = permutations.get(*sentence[1]).unwrap();

        for word1 in word1_perms {
            for word2 in word2_perms {
                if sentence.len() == 3 {
                    let word3_perms = permutations.get(*sentence[2]).unwrap();

                    for word3 in word3_perms {
                        passwords.push_back(format!("{}{}{}", word1, word2, word3));
                    }
                } else {
                    // let password = String::new()

                    passwords.push_back(format!("{}{}", word1, word2));
                }
            }
        }
    }

    let first: Vec<_> = passwords.iter().take(50).collect();
    println!("Passwords: {:?}", first);

    println!("Passwords Length: {}", passwords.len());
    println!("Guessed?: {}", passwords.contains(&"W3lc0m3B^Ck2022!".to_owned()));
}

fn generate_word_permutations<'a, 'b>(
    wordlist: &'a Vec<&'b str>,
) -> HashMap<&'b str, VecDeque<String>> {
    let lines = wordlist.len();

    let mut words_processed = 0;
    let mut passwords_generated = 0;

    let mut word_permutations: HashMap<&str, VecDeque<String>> = HashMap::new();

    for word in wordlist {
        words_processed += 1;

        let words: VecDeque<_> = generate_char_replacements(word)
            .iter()
            .flat_map(|r| generate_random_casing(r.as_str()))
            .collect();

        passwords_generated += words.len();

        word_permutations.insert(&word, words);

        if words_processed % 100 == 0 {
            println!(
                "Words Processed: {} | Passwords Generated: {} | Percent Completed: {:.2?}%",
                words_processed,
                passwords_generated,
                (words_processed as f32 / lines as f32) * 100.0
            )
        }
    }

    word_permutations
}

/**
 * Create word combinations that are 11-12 characters
 */
fn create_word_pairs<'a>(wordlist: &'a Vec<&str>) -> VecDeque<Vec<&'a &'a str>> {
    let lines = wordlist.len();

    let mut words_processed = 0;
    let mut pairs_generated = 0;

    let mut result = VecDeque::new();

    for word1 in wordlist {
        let len1 = word1.len();

        for word2 in wordlist {
            if word1 == word2 {
                continue;
            }
            
            let len2 = word2.len();

            let total_len = len1 + len2;

            if total_len > 12 {
                // Length of 2 word combo is too long
                continue;
            } else if total_len >= 9 {
                // Length of 2 word combo is just right
                result.push_back(vec![word1, word2]);

                pairs_generated += 1;
                continue;
            } else {
                // Length of 2 word combo was too short, so try a third word
                for word3 in wordlist {
                    if word1 == word3 || word2 == word3 {
                        continue;
                    }

                    let len3 = word3.len();

                    let total_len = len1 + len2 + len3;

                    if total_len > 12 {
                        // Length of 3 word combo is too long
                        continue;
                    } else if total_len > 10 {
                        // Length of 3 word combo is just right
                        result.push_back(vec![word1, word2, word3]);
                        pairs_generated += 1;
                        continue;
                    } else {
                        // Length of 3 word combo is too short so we don't care, and just skip it
                    }
                }
            }
        }

        words_processed += 1;
        println!(
            "Words Processed: {} | Pairs Generated: {} | Percent Completed: {:.2?}%",
            words_processed,
            pairs_generated,
            (words_processed as f32 / lines as f32) * 100.0
        );
    }

    result
}

fn generate_char_replacements(word: &str) -> VecDeque<String> {
    let mut res = VecDeque::new();

    if word.len() == 0 {
        return res;
    }

    if word.len() == 1 {
        let c = &word.chars().nth(0).unwrap();

        res.push_back(word.to_owned());

        let Some(replacements) = CHAR_REPLACEMENTS.get(c) else {
            return res;
       };

        for replacement in replacements {
            res.push_back(replacement.to_string())
        }

        return res;
    }

    let start = &word[..1];
    let end = &word[1..];

    let first_perms = generate_char_replacements(start);
    let end_perms = generate_char_replacements(end);

    for first_perm in first_perms {
        for end_perm in &end_perms {
            res.push_back(format!("{}{}", first_perm, end_perm));
        }
    }

    res
}

fn generate_random_casing(word: &str) -> VecDeque<String> {
    let mut res = VecDeque::new();

    if word.len() == 0 {
        return res;
    }

    if word.len() == 1 {
        let c = &word.chars().nth(0).unwrap();

        let cases = HashSet::from([c.to_ascii_uppercase(), c.to_ascii_lowercase()]);

        for case in cases {
            res.push_back(case.to_string())
        }

        return res;
    }

    let start = &word[..1];
    let end = &word[1..];

    let first_perms = generate_random_casing(start);
    let end_perms = generate_random_casing(end);

    for first_perm in first_perms {
        for end_perm in &end_perms {
            res.push_back(format!("{}{}", first_perm, end_perm));
        }
    }

    res
}

#[cfg(test)]
mod test {
    use std::collections::{HashSet, VecDeque};

    use crate::{generate_char_replacements, generate_random_casing};

    #[test]
    fn test_char_replacements() {
        const WORD: &str = "welcomeback";

        let replacements = generate_char_replacements(WORD);

        println!("{}: {:?}", WORD, replacements);

        assert!(
            replacements
                == vec![
                    "welcomeback",
                    "welcomeb4ck",
                    "welcomeb^ck",
                    "welcom3back",
                    "welcom3b4ck",
                    "welcom3b^ck",
                    "welc0meback",
                    "welc0meb4ck",
                    "welc0meb^ck",
                    "welc0m3back",
                    "welc0m3b4ck",
                    "welc0m3b^ck",
                    "w3lcomeback",
                    "w3lcomeb4ck",
                    "w3lcomeb^ck",
                    "w3lcom3back",
                    "w3lcom3b4ck",
                    "w3lcom3b^ck",
                    "w3lc0meback",
                    "w3lc0meb4ck",
                    "w3lc0meb^ck",
                    "w3lc0m3back",
                    "w3lc0m3b4ck",
                    "w3lc0m3b^ck"
                ]
        )
    }

    #[test]
    fn test_random_casing() {
        const WORD: &str = "abc";

        let random_casing = generate_random_casing(WORD);

        println!("{}: {:?}", WORD, random_casing);

        let hash_set = random_casing
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<_>>();

        assert!(hash_set == HashSet::from(["Abc", "AbC", "ABc", "ABC", "abc", "abC", "aBc", "aBC"]))
    }

    #[test]
    fn test_random_casing_and_replacement() {
        const WORD: &str = "welcomeback";

        let words: VecDeque<_> = generate_char_replacements(WORD)
            .iter()
            .flat_map(|r| generate_random_casing(r.as_str()))
            .collect();

        println!("{}: {:?}", WORD, words);

        assert!(words.contains(&String::from("W3lc0m3B^Ck")));
    }
}
