use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct SteamUser {
  steam_id: Option<String>,
  account_name: Option<String>,
  persona_name: Option<String>,
}

impl SteamUser {
  fn new() -> SteamUser {
    SteamUser {
      steam_id: None,
      account_name: None,
      persona_name: None,
    }
  }
}

fn remove_end_quotes(text: &str) -> Option<regex::Captures> {
  lazy_static! {
    static ref re: Regex = Regex::new("\"(?P<token>.*)\"").unwrap();
  }

  re.captures(text)
}

pub fn steam_vdf_parser(steam_vdf: &str) -> SteamUser {
  #[derive(Debug, PartialEq)]
  enum VDFToken {
    START,
    USER,
    KEY,
    VALUE,
    END,
  }

  let lines = steam_vdf.lines();
  let mut state: VDFToken = VDFToken::START;

  let mut steam_user: SteamUser = SteamUser::new();

  for line in lines {
    let line = line.trim();
    let tokens = line.split_whitespace();
    let mut key: String = String::new();

    for token in tokens {
      if let Some(cap) = remove_end_quotes(token) {
        let token = &cap["token"];

        match token {
          "{" => continue,
          "}" => {
            state = VDFToken::END;
          }
          "users" => {
            state = VDFToken::USER;
            continue;
          }
          _ => {
            // do nothing
          }
        }

        match &state {
          VDFToken::START => {
            state = VDFToken::KEY;
          }
          VDFToken::USER => {
            steam_user.steam_id = Some(String::from(token));
            state = VDFToken::KEY;
          }
          VDFToken::KEY => {
            key = String::from(token);
            state = VDFToken::VALUE;
          }
          VDFToken::VALUE => {
            match key.as_str() {
              "AccountName" => steam_user.account_name = Some(String::from(token)),
              "PersonaName" => steam_user.persona_name = Some(String::from(token)),
              _ => {}
            }
            state = VDFToken::KEY;
          }
          VDFToken::END => break,
        }
      }
    }

    if state == VDFToken::END {
      break;
    }
  }

  steam_user
}
