use mdbook::{Config, preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor}};

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

struct Replacer {
    list: Vec<Replacement>,
}

impl Replacer {
    fn new(ctx: &PreprocessorContext) -> Self {
        Self {
            list: get_replace_table(&ctx.config).unwrap_or_default(),
        }
    }
}

impl Preprocessor for Replacer {
    fn name(&self) -> &'static str {
        "replace"
    }

    fn run(
        &self,
        _ctx: &PreprocessorContext,
        mut book: mdbook::book::Book,
    ) -> mdbook::errors::Result<mdbook::book::Book> {
        book.for_each_mut(|item| {
            if let mdbook::book::BookItem::Chapter(chap) = item {
                for replacement in &self.list {
                    chap.content = chap.content.replace(&replacement.from, &replacement.to);
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}

fn get_replace_table(config: &Config) -> Option<Vec<Replacement>> {
    let preprocessor_config = config.get("preprocessor")?;
    let replace_config = preprocessor_config.get("replace")?;
    let table = replace_config.get("list")?;

    Some(
        table.as_table()
            .unwrap()
            .iter()
        .map(|(key, value)| Replacement {
            from: key.clone(),
            to: String::from(value.as_str().unwrap()),
        })
        .collect(),)
}

fn main() -> anyhow::Result<()> {
    if std::env::args().nth(1).as_deref() == Some("supports") {
        return Ok(());
    }

    let (ctx, book) = CmdPreprocessor::parse_input(std::io::stdin())?;
    let book = Replacer::new(&ctx).run(&ctx, book)?;
    serde_json::to_writer(std::io::stdout(), &book)?;
    Ok(())
}
