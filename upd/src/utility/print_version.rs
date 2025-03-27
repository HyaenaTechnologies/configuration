use std::process::ExitCode;

// Semantic Version Number Definition
struct SemanticVersionNumber {
    semantic_version: String,
}

// Print Version Number
pub fn print_version_number() -> ExitCode {
    let version_number: SemanticVersionNumber = SemanticVersionNumber {
        semantic_version: "0.2.0".to_string(),
    };

    println!("Systsem Update Daemon");
    println!("");
    println!("");
    println!(
        "Version Number:		{:#?}",
        version_number.semantic_version.trim()
    );

    return ExitCode::SUCCESS;
}
