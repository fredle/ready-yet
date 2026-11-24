//! How Ready Yet itself starts at sign-in. Standard mode is a per-user Run
//! entry (no admin). Early mode is a logon scheduled task registered once
//! with administrator approval, set to normal priority because tasks default
//! to below-normal.

use crate::win::{self, Hive};

const RUN: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE: &str = "Ready Yet";
pub const TASK_NAME: &str = r"\ReadyYet\Early start";
pub const COLLECT_ARG: &str = "--collect";
pub const EARLY_ARG: &str = "--early";

fn exe() -> String {
    std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn run_command(exe: &str) -> String {
    format!("\"{exe}\" {COLLECT_ARG}")
}

/// Adds or removes the standard-mode Run entry.
pub fn set_standard(on: bool) -> bool {
    if on {
        win::reg_set_text(Hive::User, RUN, VALUE, &run_command(&exe()))
    } else {
        win::reg_get_text(Hive::User, RUN, VALUE).is_none() || win::reg_delete(Hive::User, RUN, VALUE)
    }
}

pub fn standard_is_set() -> bool {
    win::reg_get_text(Hive::User, RUN, VALUE).is_some()
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// The early-mode task: at this user's sign-in, run the collector with
/// highest available rights and normal priority (4; tasks default to 7).
pub fn early_task_xml(exe: &str, user: &str) -> String {
    let exe = xml_escape(exe);
    let user = xml_escape(user);
    format!(
        r#"<?xml version="1.0" encoding="UTF-16"?>
<Task version="1.2" xmlns="http://schemas.microsoft.com/windows/2004/02/mit/task">
  <RegistrationInfo>
    <Description>Ready Yet: watches startup from the first second of sign-in, then closes.</Description>
  </RegistrationInfo>
  <Triggers>
    <LogonTrigger>
      <Enabled>true</Enabled>
      <UserId>{user}</UserId>
    </LogonTrigger>
  </Triggers>
  <Principals>
    <Principal id="Author">
      <UserId>{user}</UserId>
      <LogonType>InteractiveToken</LogonType>
      <RunLevel>HighestAvailable</RunLevel>
    </Principal>
  </Principals>
  <Settings>
    <MultipleInstancesPolicy>IgnoreNew</MultipleInstancesPolicy>
    <DisallowStartIfOnBatteries>false</DisallowStartIfOnBatteries>
    <StopIfGoingOnBatteries>false</StopIfGoingOnBatteries>
    <AllowHardTerminate>true</AllowHardTerminate>
    <StartWhenAvailable>false</StartWhenAvailable>
    <AllowStartOnDemand>true</AllowStartOnDemand>
    <Enabled>true</Enabled>
    <Hidden>false</Hidden>
    <ExecutionTimeLimit>PT15M</ExecutionTimeLimit>
    <Priority>4</Priority>
  </Settings>
  <Actions Context="Author">
    <Exec>
      <Command>{exe}</Command>
      <Arguments>{COLLECT_ARG} {EARLY_ARG}</Arguments>
    </Exec>
  </Actions>
</Task>
"#
    )
}

fn user_name() -> String {
    let user = std::env::var("USERNAME").unwrap_or_default();
    match std::env::var("USERDOMAIN") {
        Ok(d) if !d.is_empty() => format!("{d}\\{user}"),
        _ => user,
    }
}

/// Registers the early-mode task. Shows one administrator prompt. Returns a
/// message for the user on failure.
pub fn enable_early() -> Result<(), String> {
    let dir = crate::settings::data_dir();
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("early-task.xml");
    let xml = early_task_xml(&exe(), &user_name());
    let mut bytes = vec![0xFF, 0xFE];
    bytes.extend(xml.encode_utf16().flat_map(|c| c.to_le_bytes()));
    std::fs::write(&path, bytes).map_err(|e| format!("Could not write the task file: {e}"))?;
    let args = format!("/Create /TN \"{TASK_NAME}\" /XML \"{}\" /F", path.display());
    let code = win::shell_run("schtasks.exe", &args, true, true, true);
    let _ = std::fs::remove_file(&path);
    match code {
        Some(0) => {
            set_standard(false);
            Ok(())
        }
        Some(_) => Err("Windows did not accept the sign-in task. Nothing was changed.".into()),
        None => Err("Administrator approval was not given. Nothing was changed.".into()),
    }
}

/// Removes the early-mode task (one administrator prompt) and goes back to
/// the standard Run entry.
pub fn disable_early() -> Result<(), String> {
    let args = format!("/Delete /TN \"{TASK_NAME}\" /F");
    match win::shell_run("schtasks.exe", &args, true, true, true) {
        Some(_) => {
            set_standard(true);
            Ok(())
        }
        None => Err("Administrator approval was not given, so the sign-in task is still there.".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_runs_the_collector_at_normal_priority() {
        let xml = early_task_xml(r"C:\Users\a&b\ReadyYet\current\ready_yet.exe", r"PC\a&b");
        assert!(xml.contains("<Priority>4</Priority>"));
        assert!(xml.contains("<LogonTrigger>"));
        assert!(xml.contains("<RunLevel>HighestAvailable</RunLevel>"));
        assert!(xml.contains(r"<Command>C:\Users\a&amp;b\ReadyYet\current\ready_yet.exe</Command>"));
        assert!(xml.contains("<Arguments>--collect --early</Arguments>"));
        assert!(xml.contains(r"<UserId>PC\a&amp;b</UserId>"));
    }

    #[test]
    fn run_entry_quotes_the_path() {
        assert_eq!(run_command(r"C:\x y\r.exe"), r#""C:\x y\r.exe" --collect"#);
    }
}
