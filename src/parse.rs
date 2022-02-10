// MIT License
//
// Copyright (c) 2022 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod elite_cpp {
    use elite::ast::EliteKeywords;

    fn replace(data: String) -> String {
        data.replace('\"', "\\'").replace("\0", "")
    }

    pub fn parse(data: elite::parser::EliteParser) -> String {
        let mut regenerated_code = String::from("#include <iostream> \n\
                                                           #include <string> \n\
                                                           #include <filesystem> \n\
                                                           #include <cstdlib> \n\
        std::string get_arch() {
            #if defined(__x86_64__) || defined(_M_X64)
                return \"amd64\";
            #elif defined(i386) || defined(__i386__) || defined(__i386)
                return \"i386\";
            #endif
        }

        std::string get_os() {
                                                           #if defined(_WIN16)        \
        || defined(_WIN32)     \
        || defined(_WIN64)     \
        || defined(__WIN32__)  \
        || defined(__TOS_WIN__)\
        || defined(__WINDOWS__)
        return \"windows\";
    #elif defined(macintosh)   \
        || defined(Macintosh)
        || (defined(__APPLE__) && defined(__MACH__))
        return \"macos\";
    #elif defined(__linux__)    \
        || defined(linux)       \
        || defined(__linux)     \
        || defined(__gnu_linux__)
        return \"linux\";
    #elif defined(__ANDROID__)
        return \"android\";
    #elif (defined(__FreeBSD_kernel__)
        && defined(__GLIBC__))         \
        || defined(__FreeBSD__)        \
        || defined(__FreeBSD_kernel__)
        return \"freebsd\";
    #elif defined(__DragonFly__)
        return \"dragonfly\";
    #elif defined(__OpenBSD__)
        return \"openbsd\";
    #elif defined(__NetBSD__)
        return \"netbsd\";
    #endif
}\n
int main(int argc, char** argv) noexcept {\n");
        let mut line = 0u32;
        let mut is_for = false;

        for x in data.ast_nodes.data {
            match x.__type {
                EliteKeywords::Set => {
                    regenerated_code.push_str(
                        format!("{}auto {} = \"{}\";\n", " ".repeat(line as usize), x.__name, replace(x.__data)).as_str());
                }
                EliteKeywords::Print => {
                    regenerated_code.push_str(
                        format!("{}std::cout << \"{}\";\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Println => {
                    regenerated_code.push_str(
                        format!("{}std::cout << \"{}\\n\";\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Use => {}
                EliteKeywords::RequiredVersion => {
                    regenerated_code.push_str(format!("if(\"{}\" != \"{}\")\n{{\n{}",
                                                            replace(x.__name),
                                                            replace(x.__data),
                                                            " std::cout << \"elite: Required higher version\\n\";\
                                                            \n return 1;\n}\n").as_str());
                }
                EliteKeywords::Change => {}
                EliteKeywords::IfArg => {
                    regenerated_code.push_str(format!("{}if(\"{}\"", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::LeftParenthese => {}
                EliteKeywords::RightParenthese => {}
                EliteKeywords::LeftSqBracket => {
                    regenerated_code.push_str("{\n");
                    if is_for { is_for = false; continue; } line += 1;
                }
                EliteKeywords::RightSqBracket => {
                    regenerated_code.push_str("}\n");
                    if line < 1 { continue } line -= 1;
                }
                EliteKeywords::Eq => {
                    regenerated_code.push_str(format!(" == \"{}\")\n", replace(x.__data)).as_str());
                }
                EliteKeywords::UnEq => {
                    regenerated_code.push_str(format!(" != \"{}\")\n", replace(x.__data)).as_str());
                }
                EliteKeywords::Signal => {
                    if x.__name == "exit" {
                        regenerated_code.push_str(format!("{}return 1;\n", " ".repeat(line as usize)).as_str());
                    } else if x.__name == "start" {
                        is_for = true;
                    }
                }
                EliteKeywords::Exec => {
                    regenerated_code.push_str(format!("{}std::system(\"{}\");\n", " ".repeat(line as usize), replace(x.__name)).as_str());
                }
                EliteKeywords::AddSource => {}
                EliteKeywords::Append => {}
                EliteKeywords::Exit => {
                    regenerated_code.push_str(format!("{}return 1;\n", " ".repeat(line as usize)).as_str());
                }
                EliteKeywords::Specific => {
                    match x.__data.as_str() {
                        "x86" => regenerated_code.push_str(
                            format!("{}if(get_arch() == \"i386\")\n", " ".repeat(line as usize)).as_str()),
                        "amd64" => regenerated_code.push_str(
                                format!("{}if(get_arch() == \"amd64\")\n", " ".repeat(line as usize)).as_str()),
                        "windows" => regenerated_code.push_str(
                            format!("{}if(get_os() == \"windows\")\n", " ".repeat(line as usize)).as_str()),
                        "macos" => regenerated_code.push_str(
                            format!("{}if(get_os() == \"macos\")\n", " ".repeat(line as usize)).as_str()),
                        "linux" => regenerated_code.push_str(
                            format!("{}if(get_os() == \"linux\")\n", " ".repeat(line as usize)).as_str()),
                        "freebsd" => regenerated_code.push_str(
                            format!("{}if(get_os() == \"freebsd\")\n", " ".repeat(line as usize)).as_str()),
                        "netbsd" => regenerated_code.push_str(
                            format!("{}if(get_os() == \"netbsd\")", " ".repeat(line as usize)).as_str()),
                        "android" => regenerated_code.push_str(
                            format!("{}if(get_os() == \"android\")\n", " ".repeat(line as usize)).as_str()),
                        _ =>
                            // other platforms are not directly supported but this is may be TODO.
                            regenerated_code.push_str(
                            format!("{} // not supported\n", " ".repeat(line as usize)).as_str())

                    }

                }
                EliteKeywords::Argument => {
                    regenerated_code.push_str(
                        format!("{}if(argc >= 2 && std::string(argv[argc - 1]) == \"{}\")\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Exists => {
                    regenerated_code.push_str(
                        format!("{}if(std::filesystem::exists(std::filesystem::path(\"{}\")))\n", " ".repeat(line as usize), replace(x.__data)).as_str());
                }
                EliteKeywords::Undefined => {},
                _ => {}
            }
        }

        regenerated_code.push_str("}\n");
        regenerated_code
    }
}