# [elite](https://github.com/ferhatgec/elite)topp
## [elite](https://github.com/ferhatgec/elite) -> c++17 converter

### input:
```rs
required_version is 0.1

set ProjectName as "elitetopp"
set HOME        as env "HOME"

for argument "install" [
    use exec "cargo install --path ."

    for exists "{HOME}/.cargo/bin/{ProjectName}" [
        println "{ProjectName} installed to {HOME}/.cargo/bin/{ProjectName}."
    ]

    use signal "exit"
]
```

### output:
```cpp
#include <iostream> 
#include <string> 
#include <filesystem> 
#include <cstdlib> 

// get_os() and get_arch() here.

int main(int argc, char** argv) noexcept {
if("0.1" != "0.1")
{
 std::cout << "elite: Required higher version\n";
 return 1;
}
auto ProjectName = "elitetopy";
auto HOME = "/home/gech";
if(argc >= 2 && std::string(argv[argc - 1]) == "install")
{
 std::system("cargo install --path .");
 if(std::filesystem::exists(std::filesystem::path("/home/gech/.cargo/bin/elitetopy")))
{
  std::cout << "elitetopy installed to /home/gech/.cargo/bin/elitetopy.\n";
}
 return 1;
}
}

```

### elitetopp licensed under the terms of MIT License.
