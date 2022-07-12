use std::io::{self, Read};

fn main() {

    //user input
    let mut input = String::new();
    println!("what do you want ferris to say?");
    match io::stdin().read_line(&mut input){
        Ok(_) =>{
            input = input.trim().to_string(); // extra passage to remove the return at the end of the line
        }
        Err(e) =>{
            panic!("error: wrong input")
        }
    }

    //text bubble creation
    let mut textBubble = TextBubble(input);
    println!("{}" , textBubble);
    
    //ferris the crab
    println!(" \\");
    println!("  \\");
    println!("      _~^~^~_");
    println!("  \\) /  o o  \\ (/");
    println!("    '_   -   _' ");
    println!("    / '-----' \\ ");

}

fn TextBubble(text: String) -> String {

    println!("lenght = {}",text.len());

    let mut finalString:String = "".to_string();
    let mut underline;
    
    if(text.len() > 30)
    {
        underline = (0..30).map(|_| "-").collect::<String>(); //crea la linea tratteggiata sopra. ps:idk what map does specificaly
        let mut bubbleRow: Vec<String> = Vec::new();
        for i in 0..
        {
            if((i+1)*30 >= text.len())
            {
                let mut substring = text.chars().skip(i*30).take(text.len()%30).collect::<String>();
                substring = format!("{}{}",substring,(0..(30 - text.len()%30)).map(|_| " ").collect::<String>());
                bubbleRow.push(substring);
                break;
            }
            else{
                bubbleRow.push(text.chars().skip(i*30).take(30).collect::<String>());
            }
        }

        for row in bubbleRow {
            finalString = format!("{}<{}>\n",finalString,row);
        }
    }
    else
    {
        underline = (0..(text.len()%30)).map(|_| "-").collect::<String>();
        finalString = format!("<{}>\n",text);
    }

    return format!("\n{}\n{}{}",underline,finalString,underline); 
}