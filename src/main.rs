fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(mut text: String) -> String {
    let first_word: String = text.split_whitespace().next().unwrap_or("").to_string();

    text.push_str(" rust");

    let str1 = {
        let view_mut: &mut String = &mut text;
        view_mut.clone()
    };

    let str2 = {
        let view: &String = &text;
        view.clone()
    };

    format!("{first_word}\n{str1}\n{str2}\n")
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
