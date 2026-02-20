use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use directories::UserDirs;
use crate::app::App;

pub fn generate_report(app: &App) -> anyhow::Result<()> {
    // 1. Get Home Directory and set up paths
    let user_dirs = UserDirs::new().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let home_dir = user_dirs.home_dir();
    let report_dir = home_dir.join("Kanji_Report");

    // Create ~/Kanji_Report if it doesn't exist
    if !report_dir.exists() {
        fs::create_dir_all(&report_dir)?;
    }

    // 2. Construct Filenames
    let date_str = chrono::Local::now().format("%Y-%m-%d");
    let base_name = format!("{}_kanji_report", date_str);
    
    let md_path = report_dir.join(format!("{}.md", base_name));
    let pdf_path = report_dir.join(format!("{}.pdf", base_name));

    // 3. Write Markdown File
    {
        let mut file = File::create(&md_path)?;

        writeln!(file, "# Daily Kanji Report")?;
        writeln!(file, "**Date:** {}", date_str)?;
        // Use dynamic limit here
        writeln!(file, "**Score:** {}/{}\n", app.score, app.question_limit)?;

        writeln!(file, "| Question | User Answer | Correct | Result |")?;
        writeln!(file, "|---|---|---|---|")?;

        for (i, (ans, correct)) in app.user_answers.iter().enumerate() {
            if let Some(q) = app.questions.get(i) {
                let mark = if *correct { "○" } else { "×" }; 
                
                writeln!(file, "| {} | {} | {} | {} |", 
                    q.target_kanji, ans, q.correct_reading, mark)?;
            }
        }
    }

    // 4. Run Pandoc
    let status = Command::new("pandoc")
        .arg(&md_path)
        .arg("-o")
        .arg(&pdf_path)
        .arg("--pdf-engine=xelatex") 
        .arg("-V")
        .arg("mainfont=Noto Sans CJK JP") 
        .status()?;

    // 5. Cleanup and Result
    if status.success() {
        // Delete the temporary Markdown file
        if let Err(e) = fs::remove_file(&md_path) {
            eprintln!("Warning: Could not delete temporary MD file: {}", e);
        }
    } else {
        anyhow::bail!("Pandoc failed to generate PDF. Check if Pandoc/XeLaTeX is installed.");
    }

    Ok(())
}
