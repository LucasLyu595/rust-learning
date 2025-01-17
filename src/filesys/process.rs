use std::process::{Command, Stdio};

fn shell(cmd: &str) -> (String, bool) {
    // use the shell machanism for joining stderr to stdout
    let cmd = format!("{} 2>&1", cmd);
    let shell = if cfg!(target_os = "windows") {
        "cmd.exe"
    } else {
        "sh"
    };
    let flag = if cfg!(target_os = "windows") {
        "/C"
    } else {
        "-c"
    };
    let output = Command::new(shell)
        .arg(flag)
        .arg(&cmd)
        .output()
        .expect("no shell?");
    (
        String::from_utf8_lossy(&output.stdout)
            .trim_end()
            .to_string(),
        output.status.success(),
    )
}

fn shell_success(cmd: &str) -> Option<String> {
    let (output, success) = shell(cmd);
    if success {
        Some(output)
    } else {
        None
    }
}

pub fn run() {
    let status = Command::new("rustc")
        .arg("-V")
        .status() // actually trigger the command
        .expect("no rustc?");
    println!("cool {} code {}", status.success(), status.code().unwrap());
    // code can be None, when the program was killed by a signal

    let output = Command::new("rustc").arg("-V").output().expect("no rustc?");
    // As with status our program blocks until the child process is finished
    // and we get back three things
    // the status (as before), the contents of stdout and the contents of stderr
    if output.status.success() {
        println!("ok!");
    }
    println!(
        "len stdout {} len stderr {}",
        output.stdout.len(),
        output.stderr.len()
    );
    // code can be None, when the program was killed by a signal
    if let Some(res) = shell_success("rustc -V") {
        println!("cool {}", res);
    }
    let mut child = Command::new("rustc")
        .stdout(Stdio::null()) // suppress stdout
        .stderr(Stdio::null())
        .spawn() // return immediately
        .expect("no rustc?");
    // by default the child inherits the stdin, stdout and stderr of the parent
    // in this case,w e redirect the child's output handels into 'nowhere'
    // effectively saying `> /dev/null 2> /dev/null`

    let res = child.wait(); // explicitly wait for the child to finish
    println!("res {:?}", res);
}
