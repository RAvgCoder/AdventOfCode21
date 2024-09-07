use crate::day8::decoder::SignalDecoder;
use crate::utils::day_setup;
use day_setup::Utils;
use std::str::FromStr;

/// Runs the Advent of Code puzzles for [Current Day](https://adventofcode.com/2021/day/8).
///
/// This function calls the `run_part` function from the `Utils` module to execute and time
/// the solutions for both parts of the current day, checking them against the expected results.
///
/// # Panics
///   If the result of any part does not match the expected value.
pub fn run() {
    // run_part(day_func_part_to_run, part_num, day_num)
    Utils::run_part(part1, 1, 8, 344);
    Utils::run_part(part2, 2, 8, 1048410);
}

fn part1(segment_list: Vec<SignalContext>) -> u64 {
    const SEGMENTS_COUNT: [u8; 4] = ClockNumber::core_segment_counts();

    segment_list
        .iter()
        .map(|signal_segment: &SignalContext| {
            signal_segment
                .output_value
                .iter()
                .filter(|digit| {
                    let digit_len = digit.len() as u8;
                    SEGMENTS_COUNT.iter().any(|&count| count == digit_len)
                })
                .count() as u64
        })
        .sum()
}

fn part2(signal_contexts: Vec<SignalContext>) -> u64 {
    signal_contexts
        .iter()
        .map(|signal_context| {
            // [5,3,8,9] => 5389
            signal_context
                .decode()
                .into_iter()
                .fold(0, |mut acc, digit| {
                    acc *= 10;
                    acc += digit;
                    acc
                }) as u64
        })
        .sum()
}

/// Represents the numbers on a 7-segment clock display.
///
/// Each variant corresponds to a digit from 0 to 9, with the value being a bitmask
/// that indicates which segments are turned on or off.
///
/// ```
/// ++++++++++++++++++++++++
///    Segment Number
///        _0__
///       1    2
///       |    |
///        _3__
///       4    5
///       |    |
///        _6__
/// ++++++++++++++++++++++++
///
///   0:      1:      2:      3:      4:
///  aaaa    ....    aaaa    aaaa    ....
/// b    c  .    c  .    c  .    c  b    c
/// b    c  .    c  .    c  .    c  b    c
///  ....    ....    dddd    dddd    dddd
/// e    f  .    f  e    .  .    f  .    f
/// e    f  .    f  e    .  .    f  .    f
///  gggg    ....    gggg    gggg    ....
///
///   5:      6:      7:      8:      9:
///  aaaa    aaaa    aaaa    aaaa    aaaa
/// b    .  b    .  .    c  b    c  b    c
/// b    .  b    .  .    c  b    c  b    c
///  dddd    dddd    ....    dddd    dddd
/// .    f  e    f  .    f  e    f  .    f
/// .    f  e    f  .    f  e    f  .    f
///  gggg    gggg    ....    gggg    gggg
///```
#[derive(Debug, Copy, Clone)]
enum ClockNumber {
    /// Segment (0, 1, 2, 4, 5, 6)
    Zero = 0b1110111,
    /// Segment (2, 3)
    One = 0b0010010,
    /// Segment (0, 2, 3, 4, 6)
    Two = 0b1011101,
    /// Segment (0, 2, 3, 5, 6)
    Three = 0b1011011,
    /// Segment (1, 2, 3, 5)
    Four = 0b0111010,
    /// Segment (0, 1, 3, 5, 6)
    Five = 0b1101011,
    /// Segment (0, 1, 3, 4, 5, 6)
    Six = 0b1101111,
    /// Segment (1, 2, 5)
    Seven = 0b1010010,
    /// Segment (0, 1, 2, 3, 4, 5, 6)
    Eight = 0b1111111,
    /// Segment (0, 1, 2, 3, 5, 6)
    Nine = 0b1111011,
}

macro_rules! bit_index_turned_on {
    ($clock_number: expr) => {{
        const N: usize = $clock_number.count_segments() as usize;
        $clock_number.bit_index_turned_on::<{ N }>()
    }};
}

impl ClockNumber {
    /// Counts the number of segments that are turned on for the given `ClockNumber`.
    ///
    /// # Returns
    /// The number of segments that are turned on.
    ///
    /// # Example
    /// ```
    /// let clock_number = ClockNumber::Three;
    /// assert_eq!(clock_number.count_segments(), 5);
    /// ```
    #[inline(always)]
    const fn count_segments(&self) -> u8 {
        (*self as u8).count_ones() as u8
    }

    #[inline(always)]
    const fn core_segment_counts() -> [u8; 4] {
        // [ 2, 4, 3, 7 ]
        [
            ClockNumber::One.count_segments(),
            ClockNumber::Four.count_segments(),
            ClockNumber::Seven.count_segments(),
            ClockNumber::Eight.count_segments(),
        ]
    }

    /// Returns an array of bit indices that are turned on for the given `ClockNumber`.
    ///
    /// # Arguments
    /// * `self` - The `ClockNumber` instance.
    ///
    /// # Returns
    /// An array of bit indices that are turned on.
    ///
    /// # Example
    /// ```
    /// let clock_number = ClockNumber::Three;
    /// let bit_indices = clock_number.bit_index_turned_on::<5>();
    /// assert_eq!(bit_indices, [0, 2, 3, 5, 6]);
    /// ```
    const fn bit_index_turned_on<const N: usize>(self) -> [u8; N] {
        let mut bit_index = [0; N];
        let mut mask = self as u8;
        const BIT_COUNT: i32 = 7;

        let mut idx = (N - 1) as isize;
        let mut bit_count_idx = BIT_COUNT - 1;
        while mask != 0 {
            if (mask & 1) == 1 {
                bit_index[idx as usize] = bit_count_idx as u8;
                idx -= 1;
            }
            mask >>= 1;
            bit_count_idx -= 1;
        }

        bit_index
    }

    /// Returns the integer representation of the `ClockNumber`.
    ///
    /// # Returns
    /// An unsigned 8-bit integer corresponding to the `ClockNumber`.
    ///
    /// # Example
    /// ```
    /// let clock_number = ClockNumber::Three;
    /// assert_eq!(clock_number.int_repr(), 3);
    /// ```
    fn int_repr(&self) -> u8 {
        match *self {
            ClockNumber::Zero => 0,
            ClockNumber::One => 1,
            ClockNumber::Two => 2,
            ClockNumber::Three => 3,
            ClockNumber::Four => 4,
            ClockNumber::Five => 5,
            ClockNumber::Six => 6,
            ClockNumber::Seven => 7,
            ClockNumber::Eight => 8,
            ClockNumber::Nine => 9,
        }
    }
}

/// Represents the context of a signal, including unique signal patterns and output values.
///
/// # Fields
/// * `unique_signal_patterns` - An array of 10 unique signal patterns.
/// * `output_value` - An array of 4 output values.
struct SignalContext {
    unique_signal_patterns: [String; 10],
    output_value: [String; 4],
}

impl SignalContext {
    /// Decodes the output values of the signal context.
    ///
    /// This function initializes a `SignalDecoder` with the unique signal patterns,
    /// decodes the unique signal patterns, and then decodes the output values.
    ///
    /// # Returns
    /// An array of 4 decoded output values as `u16`.
    ///
    /// # Example
    /// ```
    /// let signal_context = SignalContext {
    ///     unique_signal_patterns: [String::from("ab"), String::from("cd"), ...],
    ///     output_value: [String::from("ef"), String::from("gh"), ...],
    /// };
    /// let decoded_output = signal_context.decode();
    /// assert_eq!(decoded_output, [1, 2, 3, 4]);
    /// ```
    fn decode(&self) -> [u16; 4] {
        let mut decoder_context: SignalDecoder = SignalDecoder::new(&self.unique_signal_patterns);
        decoder_context.decode_unique_signal_patterns();

        let mut decoded_output: [u16; 4] = [0; 4];

        for (idx, output) in self.output_value.iter().enumerate() {
            decoded_output[idx] = decoder_context.decode_output(output).int_repr() as u16;
        }

        decoded_output
    }
}

impl FromStr for SignalContext {
    type Err = &'static str;

    /// Parses a string input to create a `SignalContext` instance.
    ///
    /// The input string is expected to have two parts separated by a '|':
    /// the unique signal patterns and the output values. Each part contains
    /// space-separated strings representing the signal patterns and output values respectively.
    ///
    /// # Arguments
    /// * `input` - A string slice containing the unique signal patterns and output values.
    ///
    /// # Returns
    /// A `Result` containing the `SignalContext` instance if parsing is successful,
    /// or a static string slice error message if parsing fails.
    ///
    /// # Panics
    /// Panics if the input string does not contain a '|' separator.
    ///
    /// # Example
    /// ```
    /// let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    /// let signal_context = SignalContext::from_str(input).unwrap();
    /// assert_eq!(signal_context.unique_signal_patterns.len(), 10);
    /// assert_eq!(signal_context.output_value.len(), 4);
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut signal_patterns: [String; 10] = core::array::from_fn(|_| String::new());
        let mut output: [String; 4] = core::array::from_fn(|_| String::new());
        let (unique_signal_patterns, output_value) =
            input.split_once('|').expect("Invalid input format");

        unique_signal_patterns
            .split_whitespace()
            .enumerate()
            .for_each(|(idx, input)| {
                signal_patterns[idx] = input.to_string();
            });

        output_value
            .split_whitespace()
            .enumerate()
            .for_each(|(idx, input)| {
                output[idx] = input.to_string();
            });

        Ok(SignalContext {
            unique_signal_patterns: signal_patterns,
            output_value: output,
        })
    }
}

mod decoder {
    use crate::day8::ClockNumber;
    use std::collections::HashSet;

    /// Represents a digit in a 7-segment display.
    ///
    /// # Fields
    /// * `number` - The `ClockNumber` corresponding to the digit.
    /// * `segment_chars` - A set of characters representing the segments that are turned on.
    ///
    /// # Example
    /// ```
    /// let digit = Digits {
    ///     number: ClockNumber::Three,
    ///     segment_chars: HashSet::from(['a', 'b', 'c', 'd', 'e']),
    /// };
    /// assert_eq!(digit.number, ClockNumber::Three);
    /// assert!(digit.segment_chars.contains(&'a'));
    /// ```
    #[derive(Debug)]
    pub struct Digits {
        number: ClockNumber,
        segment_chars: HashSet<char>,
    }

    impl Digits {
        pub fn is_decoded(&self) -> bool {
            !self.segment_chars.is_empty()
        }
    }

    /// A struct for decoding signals in a 7-segment display.
    ///
    /// The `SignalDecoder` struct is used to decode the segments and digits of a 7-segment display
    /// based on provided signal patterns.
    #[derive(Debug)]
    pub struct SignalDecoder<'ctx> {
        /// Array to store the decoded segments (a to g) of the 7-segment display.
        pub decoded_segments: [char; 7],
        /// Array to store the decoded digits (0 to 9) with their corresponding segments.
        /// The core segments are [One, Seven, Four, Eight].
        pub decoded_digits: [Digits; 10],
        /// Reference to the signal patterns provided as input.
        pub signal_patterns: &'ctx [String; 10],
        /// Array to store the core segments with their encoded string patterns.
        /// The core segments are [One, Four, Seven, Eight].
        decoded_core_segment: [(ClockNumber, &'ctx str); 4],
    }

    impl<'ctx> SignalDecoder<'ctx> {
        pub fn decode_output(&self, output: &str) -> ClockNumber {
            let output = output.chars().collect::<HashSet<_>>();
            for decoded_digit in &self.decoded_digits {
                if decoded_digit.segment_chars.eq(&output) {
                    return decoded_digit.number;
                }
            }
            panic!("Output '{:?}' not found in signal patterns", output);
        }

        pub fn decode_unique_signal_patterns(&mut self) {
            // Decode the core segments
            for (core, pattern) in self.decoded_core_segment {
                let mut pattern = pattern.chars();
                match core {
                    ClockNumber::One => {
                        // Decode the segment
                        let bit_index = bit_index_turned_on!(ClockNumber::One);
                        for i in bit_index {
                            self.decoded_segments[i as usize] = pattern.next().unwrap();
                        }
                    }
                    ClockNumber::Seven => {
                        // Decode segment7
                        let bit_index_1 = bit_index_turned_on!(ClockNumber::One);
                        let bit_index_7 = bit_index_turned_on!(ClockNumber::Seven);

                        // set7 \ set1 = x
                        for e in pattern {
                            if self.decoded_segments[bit_index_1[0] as usize] != e
                                && self.decoded_segments[bit_index_1[1] as usize] != e
                            {
                                self.decoded_segments[bit_index_7[0] as usize] = e;
                                break;
                            }
                        }
                    }
                    ClockNumber::Four => {
                        // Decode the segment
                        let b_idx4 = bit_index_turned_on!(ClockNumber::Four);
                        let mut idx = 0; // (0, 2)

                        for e in pattern {
                            if self.decoded_segments[b_idx4[1] as usize] != e
                                && self.decoded_segments[b_idx4[3] as usize] != e
                            {
                                self.decoded_segments[b_idx4[idx] as usize] = e;
                                if idx == 2 {
                                    break;
                                } else {
                                    idx = 2;
                                }
                            }
                        }
                    }
                    ClockNumber::Eight => {
                        // Group segments with count 5 {2, 3, 5}
                        let mut bit_count_six_numbers = self
                            .signal_patterns
                            .iter()
                            .filter(|encoded_string| encoded_string.len() == 5)
                            .collect::<Vec<_>>();

                        // Find encoded_number3
                        let mut encoded3: Option<usize> = None;
                        let bc7 = bit_index_turned_on!(ClockNumber::Seven);
                        let bits = &self.decoded_segments;
                        for (idx, encoded_digits) in bit_count_six_numbers.iter().enumerate() {
                            let mut found = 0;
                            for e in encoded_digits.chars() {
                                if bits[bc7[1] as usize] == e || bits[bc7[2] as usize] == e {
                                    found += 1;
                                }
                            }
                            if found == 2 {
                                encoded3 = Some(idx);
                                break;
                            }
                        }

                        let encoded3 = bit_count_six_numbers
                            .swap_remove(encoded3.expect("Encoded number 3 not found"));
                        self.decoded_digits[3]
                            .segment_chars
                            .extend(encoded3.chars());

                        // Resolve segment 1 & 3
                        let mut third_segment: Option<char> = None;
                        let bc4 = bit_index_turned_on!(ClockNumber::Four);
                        for e in encoded3.chars() {
                            if e == bits[bc4[0] as usize] || e == bits[bc4[2] as usize] {
                                third_segment = Some(e);
                                break;
                            }
                        }

                        // Swap the segments (1, 3)
                        let third_segment = third_segment.expect("Third segment not found");
                        if third_segment != bits[bc4[2] as usize] {
                            self.decoded_segments[bc4[0] as usize] =
                                self.decoded_segments[bc4[2] as usize];
                            self.decoded_segments[bc4[2] as usize] = third_segment;
                        }

                        // Resolve segment 6
                        for e in encoded3.chars() {
                            if !self.decoded_segments.iter().any(|c| *c == e) {
                                self.decoded_segments[6] = e;
                                break;
                            }
                        }

                        // Resolve segment 2 & 5
                        let mut encoded5: Option<usize> = None;
                        let seg1 = self.decoded_segments[1];
                        for (idx, encoded_digits) in bit_count_six_numbers.iter().enumerate() {
                            if encoded_digits.chars().any(|e| e == seg1) {
                                encoded5 = Some(idx);
                                break;
                            }
                        }

                        let bits = &self.decoded_segments;
                        let encoded5 = bit_count_six_numbers
                            .swap_remove(encoded5.expect("Encoded number 3 not found"));
                        self.decoded_digits[5]
                            .segment_chars
                            .extend(encoded5.chars());
                        let bc1 = bit_index_turned_on!(ClockNumber::One);
                        let mut seg5 = None;
                        for e in encoded5.chars() {
                            if e == bits[bc1[0] as usize] || e == bits[bc1[1] as usize] {
                                seg5 = Some(e);
                                break;
                            }
                        }

                        let seg5 = seg5.expect("Segment 5 not found in digit 5");
                        if seg5 != bits[bc1[1] as usize] {
                            self.decoded_segments[bc1[0] as usize] =
                                self.decoded_segments[bc1[1] as usize];
                            self.decoded_segments[bc1[1] as usize] = seg5;
                        }

                        let digit2 = bit_count_six_numbers.pop().expect("Digit 2 not found");

                        // Resolve segment 4
                        let bits = &self.decoded_segments;
                        for e in digit2.chars() {
                            if !bits.iter().any(|c| *c == e) {
                                self.decoded_segments[4] = e;
                                break;
                            }
                        }
                    }
                    other => panic!("{:?} is not a Core segment", other),
                }
            }

            let signal_patterns = self
                .signal_patterns
                .iter()
                .map(|e| e.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            for decoded_digit in self.decoded_digits.iter_mut() {
                if !decoded_digit.is_decoded() {
                    decoded_digit
                        .segment_chars
                        .extend(match decoded_digit.number {
                            ClockNumber::Zero => {
                                let b_idx0 = bit_index_turned_on!(ClockNumber::Zero);
                                let zero = b_idx0
                                    .map(|idx| self.decoded_segments[idx as usize])
                                    .into_iter()
                                    .collect::<HashSet<_>>();

                                self.signal_patterns[Self::find_decoded(&signal_patterns, &zero)]
                                    .chars()
                            }
                            ClockNumber::Six => {
                                let b_idx6 = bit_index_turned_on!(ClockNumber::Six);
                                let six = b_idx6
                                    .map(|idx| self.decoded_segments[idx as usize])
                                    .into_iter()
                                    .collect::<HashSet<_>>();

                                self.signal_patterns[Self::find_decoded(&signal_patterns, &six)]
                                    .chars()
                            }
                            ClockNumber::Seven => {
                                let b_idx7 = bit_index_turned_on!(ClockNumber::Seven);
                                let seven = b_idx7
                                    .map(|idx| self.decoded_segments[idx as usize])
                                    .into_iter()
                                    .collect::<HashSet<_>>();

                                self.signal_patterns[Self::find_decoded(&signal_patterns, &seven)]
                                    .chars()
                            }
                            ClockNumber::Eight => {
                                let b_idx8 = bit_index_turned_on!(ClockNumber::Eight);
                                let eight = b_idx8
                                    .map(|idx| self.decoded_segments[idx as usize])
                                    .into_iter()
                                    .collect::<HashSet<_>>();

                                self.signal_patterns[Self::find_decoded(&signal_patterns, &eight)]
                                    .chars()
                            }
                            ClockNumber::Two => {
                                let b_idx2 = bit_index_turned_on!(ClockNumber::Two);
                                let two = b_idx2
                                    .map(|idx| self.decoded_segments[idx as usize])
                                    .into_iter()
                                    .collect::<HashSet<_>>();

                                self.signal_patterns[Self::find_decoded(&signal_patterns, &two)]
                                    .chars()
                            }
                            ClockNumber::Nine => {
                                let b_idx9 = bit_index_turned_on!(ClockNumber::Nine);
                                let nine = b_idx9
                                    .map(|idx| self.decoded_segments[idx as usize])
                                    .into_iter()
                                    .collect::<HashSet<_>>();

                                self.signal_patterns[Self::find_decoded(&signal_patterns, &nine)]
                                    .chars()
                            }
                            _ => panic!(
                                "Digit cannot be decoded here {:?} {:?}",
                                decoded_digit.number, decoded_digit.segment_chars
                            ),
                        });
                }
            }
        }

        fn find_decoded(
            signal_patterns: &[HashSet<char>],
            digit_segment_set: &HashSet<char>,
        ) -> usize {
            for (idx, e) in signal_patterns.iter().enumerate() {
                if e.eq(digit_segment_set) {
                    return idx;
                }
            }
            panic!("Digit not found for segment set {:?}", digit_segment_set);
        }

        pub fn new(signal_patterns: &'ctx [String; 10]) -> SignalDecoder {
            let decoded_core_segment = Self::encoded_core_segments(signal_patterns);
            let decoded_digits = [
                Digits {
                    number: ClockNumber::Zero,
                    segment_chars: HashSet::new(),
                },
                Digits {
                    number: ClockNumber::One,
                    segment_chars: HashSet::from_iter(decoded_core_segment[0].1.chars()),
                },
                Digits {
                    number: ClockNumber::Two,
                    segment_chars: HashSet::new(),
                },
                Digits {
                    number: ClockNumber::Three,
                    segment_chars: HashSet::new(),
                },
                Digits {
                    number: ClockNumber::Four,
                    segment_chars: HashSet::from_iter(decoded_core_segment[2].1.chars()),
                },
                Digits {
                    number: ClockNumber::Five,
                    segment_chars: HashSet::new(),
                },
                Digits {
                    number: ClockNumber::Six,
                    segment_chars: HashSet::new(),
                },
                Digits {
                    number: ClockNumber::Seven,
                    segment_chars: HashSet::from_iter(decoded_core_segment[1].1.chars()),
                },
                Digits {
                    number: ClockNumber::Eight,
                    segment_chars: HashSet::from_iter(decoded_core_segment[3].1.chars()),
                },
                Digits {
                    number: ClockNumber::Nine,
                    segment_chars: HashSet::new(),
                },
            ];
            Self {
                decoded_digits,
                signal_patterns,
                decoded_core_segment,
                decoded_segments: ['\0'; 7],
            }
        }

        /// Returns the core segments with their encoded string patterns.
        /// With ordering as [One, Four, Seven, Eight]
        fn encoded_core_segments(signal_patterns: &[String; 10]) -> [(ClockNumber, &str); 4] {
            let mut core_segments = [(ClockNumber::One, ""); 4];

            for segment in signal_patterns {
                match segment.len() as u8 {
                    len if len == ClockNumber::One.count_segments() => {
                        core_segments[0] = (ClockNumber::One, segment);
                    }
                    len if len == ClockNumber::Seven.count_segments() => {
                        core_segments[1] = (ClockNumber::Seven, segment);
                    }
                    len if len == ClockNumber::Four.count_segments() => {
                        core_segments[2] = (ClockNumber::Four, segment);
                    }
                    len if len == ClockNumber::Eight.count_segments() => {
                        core_segments[3] = (ClockNumber::Eight, segment);
                    }
                    _ => (/* Non-core segments */),
                }
            }

            core_segments
        }
    }
}
