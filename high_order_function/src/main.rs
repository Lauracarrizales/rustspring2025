// in-class assignment

//create a struct student(major)
struct Student {
    major:String,
 }
//Higher order functions update majors
fn update_majors(mut collection: Vec<Student>,behavior:fn(&mut Student, String), new_major:String, student_num: usize) ->Vec<Student>{
    let index = student_num - 1; // Convert 1-based to 0-based
    if index < collection.len() {
        behavior(&mut collection[index], new_major);
    } else {
        println!("Invalid student number!");
    }
    collection
}
// First order functions, assign_major(student,major_declared)
fn assign_major(s: &mut Student, major: String){
    s.major = major;
    }

fn main(){
    let students = vec![
        Student { major: "engineering".to_string() },
        Student { major: "business".to_string() },
        Student { major: "teaching".to_string() },
    ];

    println!("Original majors:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}'s major: {}", i + 1, student.major);
    }

    let students = update_majors(students, assign_major, "computer science".to_string(), 1);
    let students = update_majors(students, assign_major, "psychology".to_string(), 2);
    let students = update_majors(students, assign_major, "astronomy".to_string(), 3);   
    println!("\nUpdated majors:");
    for (i, student) in students.iter().enumerate() {
        println!("Student {}'s new major: {}", i + 1, student.major);
    }
}
