fn main() {
  let mut password_manager = PasswordManager::new("meow");
  println!("Password Manager Version: {}", password_manager.version());
  match password_manager.unlock("meow") {
    PasswordManagerVariant::Unlocked(mut unlocked_password_manager) => {
      unlocked_password_manager.add_password("user1", "password1");
      unlocked_password_manager.add_password("user2", "password2");
      unlocked_password_manager.list_passwords();
      password_manager = unlocked_password_manager.lock();
    }
    PasswordManagerVariant::Locked(still_locked_password_manager) => {
      password_manager = still_locked_password_manager;
    }
  }
  println!("Password Manager Version: {}", password_manager.version());
}

use std::collections::HashMap;
struct Locked;
struct Unlocked;
struct PasswordManager<S = Locked> {
  master_password: String,
  passwords: std::collections::HashMap<String, String>,
  _marker: std::marker::PhantomData<S>,
}

enum PasswordManagerVariant {
  Locked(PasswordManager<Locked>),
  Unlocked(PasswordManager<Unlocked>),
}

impl PasswordManager {
  fn new(master_password: &str) -> PasswordManager {
    PasswordManager {
      master_password: master_password.to_string(),
      passwords: HashMap::new(),
      _marker: std::marker::PhantomData,
    }
  }
}

impl<S> PasswordManager<S> {
  fn version(&self) -> &str {
    "0.1.0"
  }
}

impl<Locked> PasswordManager<Locked> {
  fn unlock(self, master_password: &str) -> PasswordManagerVariant {
    if self.master_password == master_password {
      PasswordManagerVariant::Unlocked(PasswordManager {
        master_password: self.master_password,
        passwords: self.passwords,
        _marker: std::marker::PhantomData,
      })
    } else {
      PasswordManagerVariant::Locked(PasswordManager {
        master_password: self.master_password,
        passwords: self.passwords,
        _marker: std::marker::PhantomData,
      })
    }
  }
}

impl<Unlocked> PasswordManager<Unlocked> {
  fn lock(self) -> PasswordManager<Locked> {
    PasswordManager {
      master_password: self.master_password,
      passwords: self.passwords,
      _marker: std::marker::PhantomData,
    }
  }

  fn add_password(&mut self, username: &str, password: &str) {
    self
      .passwords
      .insert(username.to_string(), password.to_string());
  }

  fn list_passwords(&self) {
    for (username, _) in &self.passwords {
      println!("{}", username);
    }
  }
}
