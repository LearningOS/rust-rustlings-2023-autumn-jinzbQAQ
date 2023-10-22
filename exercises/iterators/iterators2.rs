// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(s: &str) -> String {
    if s.is_empty(){
        return s.to_string();
    }
    let mut d =s.chars();
    let a = d.next().unwrap().to_uppercase().collect::<String>();
    let b:String = d.collect();
    let mut c = String::new();
    c.push_str(&a);
    c.push_str(&b);
    c
    
    
    
    
    
    // if s.is_empty() {
    //     return String::from(s);
    // }

    // let mut chars = s.chars();
    // let first_char = chars.next().unwrap().to_uppercase().collect::<String>();
    // let rest_of_string: String = chars.collect();


    // let mut result = String::new();
    // result.push_str(&first_char);
    // result.push_str(&rest_of_string);

    // result
    //  if input.is_empty(){
    //     return input.to_string();
    //  }
    //  let a = input.chars().next().unwrap();
    //  let a= a.to_uppercase();
    //  let b = &input[1..];
    //  let mut c = String::new();
    //  c.push_str(a);
    //  c.push_str(b);
    //  c

    //  match c.next() {
    //     None => String::new(),
    //     Some(first) => first.to_string().to_uppercase(),
    //  };
    // let mut x=String::new();
    // for i in input.chars {
    //     x.push(i);
    // }
    
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut i = Vec::new();
    for j in words{
        i.push(capitalize_first(&capitalize_first(j)));
    }
    i
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut x=String::new();
    for i in words{
        x.push_str(&capitalize_first(i));
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
