fn main() {
    let s = String::from("SELECT   b.id,
                        b.name,
                        b.time_created,
                        b.url,
                        b.stamp,
                        b.rev_no
                    FROM bookmarkt.bookmark b
                    WHERE
                        b.id = {{ id }}
                    ;");


    //Imprime les valeurs en bytes
    /*
        for b in s.as_bytes() {
            print!("{}, ", b);
        }
    */


    //Imprime les valeurs en caract�re
    /*
    for c in s.chars() {
      print!("{}, ", c);
    }
    */

    /* debut algo detect
    for char in s.chars(){

        println!("{}, ", char);

         if(char == '{'  ){

            println!("d�tecter d�but");
         }

         if(char == '}'){

             println!("d�tecter fin");
         }
    }
    */


    /*

      let s_length = s.len();
      let s_bytes = s.as_bytes();



      for i in 0 .. s_length {

         println!("{}", s_bytes[i] as char);

      }

    */


    let id: i32 = 1;
    let changed_s = s.replace("{{ id }}", &id.to_string() );

    print!("{}",changed_s);







}
