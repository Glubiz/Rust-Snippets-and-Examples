use std::collections::HashMap;
use std::fs;

pub fn render_template(template: &str, context: &HashMap<String, String>) -> String {
    let mut output = template.to_owned();

    for (key, value) in context.iter() {
        let placeholder = format!("{{{{{}}}}}", key);
        output = output.replace(&placeholder, value);
    }

    output
}

fn main() {
    let template = fs::read_to_string("template.html").unwrap();
    let mut context = HashMap::new();
    context.insert("title".to_string(), "My Title".to_string());
    context.insert("body".to_string(), "Hello, world!".to_string());

    let output = render_template(&template, &context);
    println!("{}", output);
}
