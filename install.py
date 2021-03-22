import os
import platform
from shutil import copyfile

install_path = "install/pass/"
data_file = install_path + ".data"
bin_path = "target/release/"
app_name = "pass"

def get_extension():
    sys_name = platform.system()
    if sys_name == "Windows":
        return ".exe"
    elif sys_name == "Linux":
        return ""
    elif sys_name == "Darwin":
        return ".app"

    return ""

def main():
    os.system("cargo build --release")

    if not os.path.exists(install_path):
        os.makedirs(install_path)
    
    if not os.path.exists(data_file):
        f = open(data_file, "w")
        f.close()

    executable = app_name + get_extension()
    copyfile(bin_path + executable, install_path + executable)

if __name__ == "__main__":
    main()
