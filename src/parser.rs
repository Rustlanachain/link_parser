use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Link {
    pub href: String,
    pub text: String,
}

pub fn generate_links(html_doc: &str) -> Vec<Link> {
    let document = Html::parse_document(html_doc);
    let selector = Selector::parse("a").unwrap();
    let mut return_data = Vec::new();

    for element in document.select(&selector) {
        return_data.push(Link {
            href: element.value().attr("href").unwrap_or("").to_string(),
            text: element
                .text()
                .fold(String::new(), |l, r| l + r.trim() + " ")
                .trim_end()
                .to_string(),
        });
    }

    return_data
}

// 1st option: if we didn't need to remove extra space
// println!("{}", element.text().collect::<Vec<_>>().join(" "));

// 2nd option: using flat_map but unnecessary vector creation
// let s: String = element
//     .text()
//     .flat_map(|s| vec![format!("{} ", s.trim())])
//     .collect();
// println!("{}", s);

// 3rd option: better performance than simple concatention but more code
// println!(
//     "{}",
//     element.text().fold(String::new(), |mut a, b| {
//         let c = b.trim();
//         a.reserve(c.len() + 1);
//         a.push_str(c);
//         a.push_str(" ");
//         a
//     })
// );
