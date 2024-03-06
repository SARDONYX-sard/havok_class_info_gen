use std::fmt::Write;

#[derive(Debug, Default, PartialEq)]
pub struct HkArray<T> {
    pub numelements: usize,
    value: Vec<T>,
}

impl<T: std::fmt::Debug> HkArray<T> {
    pub fn new(numelements: usize, value: Vec<Vec<T>>) -> Self {
        Self { numelements, value }
    }

    pub fn serialize(&self) -> String
    where
        T: ToString,
    {
        let mut result = String::new();
        write!(&mut result, "{}", self.numelements).expect("Failed to write");
        for inner_vec in &self.value {
            write!(&mut result, "  (").expect("Failed to write");
            for item in inner_vec {
                write!(&mut result, "{} ", item.to_string()).expect("Failed to write");
            }
            write!(&mut result, ")\n").expect("Failed to write");
        }
        result
    }

    pub fn deserialize(input: &str) -> Option<Self>
    where
        T: std::str::FromStr,
    {
        let mut lines = input.lines();
        let numelements: usize = match lines.next()?.trim().parse().ok() {
            Some(num) => num,
            None => return None,
        };

        let mut value = Vec::new();
        for line in lines {
            let mut inner_vec = Vec::new();
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            for token in trimmed_line.split_whitespace() {
                match token.parse().ok() {
                    Some(parsed_value) => inner_vec.push(parsed_value),
                    None => return None,
                }
            }
            value.push(inner_vec);
        }

        Some(Self { numelements, value })
    }
}
