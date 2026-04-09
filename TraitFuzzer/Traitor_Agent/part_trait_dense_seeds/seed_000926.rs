use std::env;
use std::result::Result;

trait EnvVarExt {
    fn get_env_var(&self, key: &str) -> Result<String, env::VarError>;
}

impl EnvVarExt for () {
    fn get_env_var(&self, key: &str) -> Result<String, env::VarError> {
        env::var(key)
    }
}

pub fn main() {
    assert_eq!(().get_env_var("TEST_EXEC_ENV"), Ok("22".to_string()));
}