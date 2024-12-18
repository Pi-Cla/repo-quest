use std::{collections::HashMap, ops::Deref, path::Path, process::Command, sync::LazyLock};

use cfg_if::cfg_if;

#[cfg(unix)]
fn get_user_env() -> HashMap<String, String> {
  use std::env;

  let shell = env::var("SHELL").unwrap_or_else(|_| String::from("sh"));
  let output = Command::new(shell)
    .args(["-c", "env"])
    .output()
    .expect("Failed to get shell env");
  let stdout = String::from_utf8(output.stdout).expect("Env vars not utf8");
  let mut key_vals: HashMap <String,String> = HashMap::new();
  for line in stdout.lines() {
    // If a key-value pair can be parsed we insert it, else we move on
    if let Some((key, value)) = line.split_once("=") {
      key_vals.insert(key.to_string(), value.to_string());
    }
  }
  key_vals
}

static ENV: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
  cfg_if! {
      if #[cfg(unix)] {
        get_user_env()
      } else {
        HashMap::default()
      }
  }
});

pub fn command(args: &str, dir: &Path) -> Command {
  let mut arg_vec = shlex::split(args).expect("Invalid command");
  let mut cmd = Command::new(arg_vec.remove(0));
  cmd.current_dir(dir).envs(ENV.deref()).args(arg_vec);
  cmd
}
