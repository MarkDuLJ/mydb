use mydb::sql;

fn main() {
    let token = sql::token::Token::Keyword(sql::token::Keyword::Create);
    println!("{}", token);
}