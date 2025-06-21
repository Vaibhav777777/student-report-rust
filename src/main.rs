use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::io;
use anyhow::Result;

fn main() -> Result<()> {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name)?;
    println!("Enter total marks obtained:");
    io::stdin().read_line(&mut total_marks)?;
    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects)?;

    let name = name.trim();
    let total_marks: f64 = total_marks.trim().parse()?;
    let num_subjects: f64 = num_subjects.trim().parse()?;

    let average = calculate_average(total_marks, num_subjects);
    let grade = assign_grade(average);

    println!("\nGenerating report card...");
    generate_pdf(name, total_marks, num_subjects, average, &grade)?;

    println!("Report card generated as 'report_card.pdf'");
    Ok(())
}

fn calculate_average(total: f64, subjects: f64) -> f64 {
    total / subjects
}

fn assign_grade(avg: f64) -> String {
    match avg {
        a if a >= 90.0 => "A".to_string(),
        a if a >= 75.0 => "B".to_string(),
        a if a >= 60.0 => "C".to_string(),
        _ => "D".to_string(),
    }
}

fn generate_pdf(name: &str, total: f64, subjects: f64, avg: f64, grade: &str) -> Result<()> {
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    let (doc, page1, layer1) = PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let font_size = 14.0;
    let start_x = Mm(20.0);
    let mut y = Mm(250.0); // Start near top of page
    let line_spacing = Mm(10.0); // Space between lines

    let lines = vec![
        "Student Report Card".to_string(),
        format!("Name        : {}", name),
        format!("Total Marks : {}", total),
        format!("Subjects    : {}", subjects),
        format!("Average     : {:.2}", avg),
        format!("Grade       : {}", grade),
    ];

    for line in lines {
        current_layer.use_text(line, font_size, start_x, y, &font);
        y -= line_spacing;
    }

    doc.save(&mut BufWriter::new(File::create("report_card.pdf")?))?;
    Ok(())
}