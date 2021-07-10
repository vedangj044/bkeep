use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use strum_macros::{AsRefStr, EnumString};

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Parameters {
    add,
    list,
    tick,
    remove,
}

#[derive(AsRefStr, EnumString)]
#[allow(non_camel_case_types)]
pub enum Argv {
    book_name,
    all,
}

pub fn get_clap_app() -> ArgMatches<'static> {
    let book_name_arg = Arg::with_name(Argv::book_name.as_ref())
        .required(true)
        .help("Name of the book.");

    let all_flag = Arg::with_name(Argv::all.as_ref())
        .long(Argv::all.as_ref())
        .help("List books marked as read.");

    App::new("bkeep")
        .version("1.0")
        .author("Vedang Joshi <vedangj044@gmail.com")
        .about("Helps you manange your book reading list through CLI.")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name(&Parameters::add.as_ref())
                .about("Add the book to your reading list.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(&book_name_arg),
        )
        .subcommand(
            SubCommand::with_name(&Parameters::list.as_ref())
                .about("List of books in your reading list.")
                .arg(all_flag),
        )
        .subcommand(
            SubCommand::with_name(&Parameters::remove.as_ref())
                .about("Remove a book from reading list.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(&book_name_arg),
        )
        .subcommand(
            SubCommand::with_name(&Parameters::tick.as_ref())
                .about("Mark the book as read.")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(&book_name_arg),
        )
        .get_matches()
}
