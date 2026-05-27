use std::env;
use std::process::Command;

struct Task<'a> {
    name: &'a str,
    command: &'a str,
    args: Vec<&'a str>,
}

fn run_task(task: &Task) -> Result<(), String> {
    println!("[*] بدء التنفيذ: {}", task.name);
    
    let output = Command::new(task.command)
        .args(&task.args)
        .output()
        .map_err(|e| format!("فشل تهيئة الأمر {}: {}", task.command, e))?;

    if output.status.success() {
        println!("[+] اكتمل: {}", task.name);
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(format!("خطأ في مرحلة {}: {}", task.name, err))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("طريقة الاستخدام: cargo run <target.com>");
        std::process::exit(1);
    }

    let target = &args[1];
    let subdomains_file = format!("{}_subdomains.txt", target);
    let alive_file = format!("{}_alive.txt", target);
    let nuclei_file = format!("{}_nuclei.txt", target);

    let tasks = vec![
        Task {
            name: "Subfinder (استخراج النطاقات)",
            command: "subfinder",
            args: vec!["-d", target, "-o", &subdomains_file, "-silent"],
        },
        Task {
            name: "HTTPX (فحص النطاقات الحية)",
            command: "httpx",
            args: vec!["-l", &subdomains_file, "-o", &alive_file, "-silent"],
        },
        Task {
            name: "Nuclei (فحص الثغرات)",
            command: "nuclei",
            args: vec!["-l", &alive_file, "-o", &nuclei_file, "-severity", "low,medium,high,critical"],
        },
    ];

    for task in tasks {
        match run_task(&task) {
            Ok(_) => continue,
            Err(e) => {
                eprintln!("[-] توقف سير العمل. {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("[+] اكتمل سير العمل بنجاح للهدف: {}", target);
}
